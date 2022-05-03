use crate::event::loop_control::LoopControl;
use crate::event::loop_state::LoopStateRef;
use crate::event::loop_target::LoopTarget;
use crate::event::queue::Queue;
use crate::event::{Event, Flow, InnerLoop};
use crate::{event, platform};
use std::process::exit;
use std::sync::RwLock;

pub struct MainLoop {
    state: LoopStateRef,
    pub(super) child_loop_controls: RwLock<Vec<LoopControl>>,
    pub(crate) inner: platform::event::MainLoop,
}
impl MainLoop {
    pub fn new() -> Self {
        MainLoop {
            state: LoopStateRef::new(),
            child_loop_controls: RwLock::new(vec![]),
            inner: platform::event::MainLoop::new(),
        }
    }

    pub fn run<'main>(
        &'main mut self,
        mut callback: impl FnMut(&LoopTarget<'main, 'main>, Option<&Event>, &mut Flow),
    ) -> ! {
        let mut flow = Flow::Wait;
        let mut inner = platform::event::MainLoop::new();
        let target = LoopTarget::Main(self);
        let exit_code = loop {
            if let Flow::Exit(exit_code) = flow {
                break exit_code;
            }
            let events = inner.process(&flow) as &mut dyn Queue<Event>;
            for event in events.as_iter_mut() {
                callback(&target, Some(&event), &mut flow);
            }
        };
        {
            let mut child_controls = self.child_loop_controls.write().unwrap();
            for mut ctx in child_controls.drain(..) {
                ctx.signal_exit();
                let exit_code = ctx.join();
                if !exit_code.is_success() {
                    exit(exit_code.into());
                }
            }
        }
        exit(exit_code.into())
    }
}
