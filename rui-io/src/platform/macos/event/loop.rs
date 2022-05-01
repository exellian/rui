use std::collections::VecDeque;
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

    // On macos events can only happen on the main thread
    // Therefore we just return an empty VecDeque
    fn process(&self, _: &Flow) -> VecDeque<Event> {
        VecDeque::from([])
    }
}
