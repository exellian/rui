use crate::event::{Event, Flow, InnerLoop};
use std::collections::VecDeque;

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

    fn init(&mut self) {
        todo!()
    }

    fn process(&mut self, flow: &Flow) -> VecDeque<Event> {
        todo!()
    }
}
