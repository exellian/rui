use crate::event::{Event, Flow, InnerLoop};
use crate::event::exit_code::ExitCode;
use crate::event::loop_state::LoopStateRef;
use crate::event::loop_target::LoopTarget;
use crate::event::main_loop::MainLoop;
use crate::platform;

/// Loop is the main handle to deal with a event loop.
pub struct Loop<'main> {
    /// This reference to the MainLoop is necessary because some operating system require to perform
    /// some task exclusively on the main thread.
    pub(super) main: &'main MainLoop,
    state: LoopStateRef,
    inner: platform::event::Loop
}

impl<'main> Loop<'main> {
    /// Creates a new Loop
    ///
    /// # Parameter
    ///
    ///  - [main: &'main MainLoop](MainLoop) A reference to the main loop which is necessary on serveral operating
    ///    systems as they require certain tasks to be run on the main thread.
    ///  - [state: LoopStateRef](LoopStateRef) may be used in conjunction with [`crate::event::loop_control::LoopControl`]
    ///    to have a thread-safe way of starting and stopping the loop.
    pub fn new(main: &'main MainLoop, state: LoopStateRef) -> Self {
        Loop {
            main,
            state,
            inner: platform::event::Loop::new()
        }
    }

    /// Processes events and calls the callback function if necessary
    ///
    /// # Parameter
    ///
    ///  - [&'child mut self](mut self) Requires a mutable reference to self
    ///  // TODO!
    pub fn run<'child>(&'child mut self, mut callback: impl FnMut(&LoopTarget<'main, 'child>, Option<&Event>, &mut Flow)) -> ExitCode where 'child: 'main {
        let mut flow = Flow::Wait;
        let target = LoopTarget::Child(self);
        self.state.start_weak();
        let exit_code = loop {
            if let Flow::Exit(exit_code) = flow {
                break exit_code
            }
            if !self.state.is_running() {
                break ExitCode::Default
            }
            let events = self.inner.process(&flow);
            for event in &events {
                callback(&target, Some(event), &mut flow);
            }
        };
        exit_code
    }
}