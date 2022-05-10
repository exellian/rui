use crate::event::{Event, Flow, InnerLoop};

pub struct Loop {}

impl Loop {
    pub fn new() -> Self {
        Loop {}
    }
}

impl InnerLoop for Loop {
    fn wake_up(&self) {
        todo!()
    }

    fn init(&mut self, _callback: impl FnMut(&Event)) {
        todo!()
    }

    fn process(&mut self, _flow: &Flow) {
        todo!()
    }
}
