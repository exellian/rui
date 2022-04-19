use crate::alloc::spmc::Sender;
use crate::event::Event;

pub struct ThreadQueue {

}

impl ThreadQueue {
    fn new() -> Self {
        ThreadQueue {}
    }

    pub fn wait(&mut self, sender: &mut Sender<Event>) {

    }

    pub fn poll(&mut self, sender: &mut Sender<Event>) {

    }
}

impl Default for ThreadQueue {
    fn default() -> Self {
        ThreadQueue::new()
    }
}
