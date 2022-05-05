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

    fn init(&mut self, callback: impl FnMut(&Event)) {}

    // On macos events can only happen on the main thread
    // Therefore we just return an empty VecDeque
    fn process(&mut self, _: &Flow) {}
}
