use crate::event::inner::InnerFlow;
use crate::event::Event;
use super::Loop;

pub struct Child {
    pub(crate) inner: Loop,
}
impl Child {
    pub fn new() -> Self {
        Child { inner: Loop::new() }
    }
}
impl crate::event::inner::InnerLoop for Child {
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
