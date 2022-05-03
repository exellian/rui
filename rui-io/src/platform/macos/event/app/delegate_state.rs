use crate::event::queue::Enqueue;
use crate::event::Event;
use crate::platform::event::Queue;

pub struct DelegateState {
    queue: Queue,
}
impl DelegateState {
    pub fn new(queue: Queue) -> Self {
        DelegateState { queue }
    }

    pub fn did_finish_launching(&mut self) {
        self.queue.enqueue(Event::Default)
    }
}
