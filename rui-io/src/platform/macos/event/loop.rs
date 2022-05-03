use crate::event::{Flow, InnerLoop};
use crate::platform::event::none_queue::NoneQueue;

pub struct Loop {
    queue: NoneQueue,
}
impl Loop {
    pub fn new() -> Self {
        Loop {
            queue: NoneQueue::new(),
        }
    }
}
impl InnerLoop for Loop {
    type Queue = NoneQueue;

    fn wake_up(&self) {
        todo!()
    }

    // On macos events can only happen on the main thread
    // Therefore we just return an empty VecDeque
    fn process(&mut self, _: &Flow) -> &mut Self::Queue {
        &mut self.queue
    }
}
