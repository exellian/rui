use crate::event::exit_code::ExitCode;
use crate::event::loop_state::LoopStateRef;
use crate::event::loop_target::LoopTarget;
use crate::event::main_loop::MainLoop;
use crate::event::{Event, Flow, InnerLoop, Queue};
use crate::platform;

/// Loop is the handle to for a child event loop. Read more about the relationship between the main
/// and the child loop in the documentation of the [MainLoop].
pub struct Loop<'main> {
    /// This reference to the MainLoop is necessary because some operating system require to perform
    /// some task exclusively on the main thread.
    pub(crate) main: &'main MainLoop,
    state: LoopStateRef,
}

impl<'main> Loop<'main> {
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
        Loop { main, state }
    }

    /// Processes events and calls the callback function if necessary
    ///
    /// # Parameter
    ///
    ///  - [&'child mut self](mut self) Requires a mutable reference to self
    ///  // TODO!
    pub fn run<'child>(
        self: &'child mut Self,
        mut callback: impl FnMut(&LoopTarget<'main, 'child>, Option<&Event>, &mut Flow),
    ) -> ExitCode
    where
        'child: 'main,
    {
        let mut inner = platform::event::Loop::new();
        let mut flow = Flow::Wait;
        let mut target = LoopTarget::Child(self);
        self.state.start_weak();
        let exit_code = loop {
            if let Flow::Exit(exit_code) = flow {
                break exit_code;
            }
            if !self.state.is_running() {
                break ExitCode::Default;
            }
            let events = inner.process(&flow) as &mut dyn Queue<Event>;
            for event in events.as_iter_mut() {
                callback(&target, Some(&event), &mut flow);
            }
        };
        exit_code
    }
}
