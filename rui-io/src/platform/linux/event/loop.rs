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

    fn process(&self, _: &Flow) -> VecDeque<Event> {
        todo!()
    }
}
