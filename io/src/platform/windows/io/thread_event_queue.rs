use crate::alloc::spmc::Sender;
use crate::event::Event;

pub struct ThreadEventQueue {

}

impl ThreadEventQueue {
    fn new() -> Self {
        ThreadEventQueue {}
    }

    pub fn wait(&mut self, sender: &mut Sender<Event>) {

    }

    pub fn poll(&mut self, sender: &mut Sender<Event>) {

    }
}

impl Default for ThreadEventQueue {
    fn default() -> Self {
        ThreadEventQueue::new()
    }
}
