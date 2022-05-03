use crate::event::{Event, Queue};

#[derive(Clone)]
pub struct NoneQueue;
impl NoneQueue {
    pub fn new() -> Self {
        NoneQueue
    }
}

impl crate::event::queue::Enqueue<Event> for NoneQueue {
    fn enqueue(&mut self, x: Event) {}
}

impl crate::event::queue::Dequeue<Event> for NoneQueue {
    fn dequeue(&mut self) -> Option<Event> {
        None
    }
}

impl Queue<Event> for NoneQueue {}
