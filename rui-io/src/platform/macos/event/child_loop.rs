use crate::event::inner::{InnerFlow, InnerLoop};
use crate::event::Event;

pub struct ChildLoop {}
impl ChildLoop {
    pub fn new() -> Self {
        ChildLoop {}
    }
}
impl InnerLoop for ChildLoop {
    fn wake_up(&self) {
        todo!()
    }

    fn init(&mut self, callback: impl FnMut(&Event)) {}

    fn process(&mut self, _: &InnerFlow) {}
}
