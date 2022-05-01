use crate::event::{Event, LoopTarget};

pub fn queue_event(loop_target: &LoopTarget, event: Event) {
    match *loop_target {
        LoopTarget::Main(main_loop) => {
            main_loop.inner.queue_event(event)
        }
        LoopTarget::Child(_) => unreachable!()
    }
}