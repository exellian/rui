use crate::event::exit_code::ExitCode;
use crate::event::inner::InnerLoop;
use crate::event::loop_state::LoopStateRef;
use crate::event::loop_target::LoopTarget;
use crate::event::main_loop::MainLoop;
use crate::event::{Event, Flow};
use crate::platform;
use std::cell::RefCell;

/// Loop is the handle to for a child event loop. Read more about the relationship between the main
/// and the child loop in the documentation of the [MainLoop].
pub struct ChildLoop<'main> {
    /// This reference to the MainLoop is necessary because some operating system require to perform
    /// some task exclusively on the main thread.
    pub(crate) main: &'main MainLoop,
    pub(crate) inner: RefCell<platform::event::ChildLoop>,
    state: LoopStateRef,
}

impl<'main> ChildLoop<'main> {
    /// Creates a new Loop
    ///
    /// # Parameter
    ///
    ///  - [main: &'main MainLoop](MainLoop) A reference to the main loop which is necessary on serveral operating
    ///    systems as they require certain tasks to be run on the main thread.
    ///    Furthermore the lifetime bound ensures that each [Loop] does not outlive the [MainLoop].
    ///    This grants us the possibility to borrow data from other threads.
    ///  - [state: LoopStateRef](LoopStateRef) may be used in conjunction with [`crate::event::loop_control::LoopControl`]
    ///    to have a thread-safe way of starting and stopping the loop.
    pub fn new(main: &'main MainLoop, state: LoopStateRef) -> Self {
        ChildLoop {
            main,
            inner: RefCell::new(platform::event::ChildLoop::new()),
            state,
        }
    }

    /// Processes events and calls the callback function if necessary
    ///
    /// # Parameter
    ///
    ///  - [&'child mut self](mut self) Requires a mutable reference to self
    ///  // TODO!
    #[allow(unused_mut)]
    pub fn run<'child>(
        self: &'child mut Self,
        mut callback: impl FnMut(&LoopTarget<'main, 'child>, Option<&Event>, &mut Flow),
    ) -> ExitCode
    where
        'child: 'main,
    {
        self.state.start_weak();

        let mut flow = Flow::Wait;
        let target = LoopTarget::Child(self);
        let mut emitted = false;
        self.inner.borrow_mut().init(|event| {
            callback(&target, Some(event), &mut flow);
            emitted = true;
        });
        let exit_code = loop {
            if let Flow::Exit(exit_code) = flow {
                break exit_code;
            }
            {
                let mut mut_guard = self.inner.borrow_mut();
                mut_guard.process(&flow.clone().try_into().unwrap());
            }
            if !emitted {
                callback(&target, None, &mut flow);
            }
            // Reset emitted boolean to check if an event was emitted
            emitted = false;
        };
        exit_code
    }
}
