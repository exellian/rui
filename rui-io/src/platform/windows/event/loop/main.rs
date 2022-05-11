use crate::event::inner::InnerFlow;
use crate::event::{Event, Flow};
use super::Loop;

pub struct Main {
    pub(crate) inner: Loop,
}
impl Main {
    pub fn new() -> Self {
        Main { inner: Loop::new() }
    }
}
impl crate::event::inner::InnerLoop for Main {
    fn wake_up(&self) {
        self.inner.wake_up()
    }

    fn init(&mut self, callback: impl FnMut(&Event)) {
        self.inner.init(callback)
    }

    fn process(&mut self, flow: &InnerFlow) {
        self.inner.process(flow)
    }
}
