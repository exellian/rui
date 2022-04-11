use crate::alloc::spmc::Sender;
use crate::event::Event;

pub struct IO {

}
impl IO {

    fn new() -> Self {
        IO {}
    }

    pub fn wait(&mut self, sender: &mut Sender<Event>) {

    }

    pub fn poll(&mut self, sender: &mut Sender<Event>) {

    }
}
impl Default for IO {
    fn default() -> Self {
        IO::new()
    }
}