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

    fn init(&mut self, _callback: impl FnMut(&Event)) {
        todo!()
    }

    fn process(&mut self, _flow: &InnerFlow) {
        todo!()
    }
}
