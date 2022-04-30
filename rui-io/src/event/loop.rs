use crate::event::{Event, Flow, InnerLoop};
use crate::event::exit_code::ExitCode;
use crate::event::loop_state::LoopStateRef;
use crate::event::loop_target::LoopTarget;
use crate::event::main_loop::MainLoop;
use crate::platform;

pub struct Loop<'main> {
    pub(super) main: &'main MainLoop,
    state: LoopStateRef,
    inner: platform::event::Loop
}
impl<'main> Loop<'main> {

    pub fn new(main: &'main MainLoop, state: LoopStateRef) -> Self {
        Loop {
            main,
            state,
            inner: platform::event::Loop::new()
        }
    }

    pub fn run<'child>(&'child self, mut callback: impl FnMut(&LoopTarget<'main, 'child>, Option<&Event>, &mut Flow)) -> ExitCode where 'child: 'main {
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
            self.inner.process(&flow, |event| {
                callback(&target, event, &mut flow);
            });
        };
        exit_code
    }
}