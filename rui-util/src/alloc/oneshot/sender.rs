use crate::alloc::oneshot::channel::Channel;
use std::sync::Arc;

pub struct Sender<T> {
    inner: Arc<Channel<T>>,
}
impl<T> Sender<T> {
    pub(crate) fn new(inner: Arc<Channel<T>>) -> Self {
        Sender { inner }
    }

    pub fn send(self, x: T) {
        unsafe { self.inner.send(x) }
        //self gets dropped here therefore send can't be called again
    }
}
