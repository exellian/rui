use crate::event::loop_control::LoopControl;
use crate::event::loop_state::LoopStateRef;
use crate::event::loop_target::LoopTarget;
use crate::event::queue::Queue;
use crate::event::{Event, Flow, InnerLoop};
use crate::platform;
use std::cell::{Ref, RefCell};
use std::process::exit;
use std::sync::RwLock;

pub struct MainLoop {
    state: LoopStateRef,
    pub(super) child_loop_controls: RwLock<Vec<LoopControl>>,
    pub(crate) inner: RefCell<platform::event::MainLoop>,
}
impl MainLoop {
    pub fn new() -> Self {
        MainLoop {
            state: LoopStateRef::new(),
            child_loop_controls: RwLock::new(vec![]),
            inner: RefCell::new(platform::event::MainLoop::new()),
        }
    }

    pub fn run<'main>(
        &'main mut self,
        mut callback: impl FnMut(&LoopTarget<'main, 'main>, Option<&Event>, &mut Flow),
    ) -> ! {
        let mut flow = Flow::Wait;
        let target = LoopTarget::Main(self);
        let exit_code = loop {
            if let Flow::Exit(exit_code) = flow {
                break exit_code;
            }
            let events: Vec<Event> = {
                let mut mut_guard = self.inner.borrow_mut();
                (mut_guard.process(&flow) as &mut dyn Queue<Event>)
                    .as_iter_mut()
                    .collect()
            };
            let event_count = events.len();
            for event in events {
                callback(&target, Some(&event), &mut flow);
            }
            if event_count == 0 {
                callback(&target, None, &mut flow);
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
