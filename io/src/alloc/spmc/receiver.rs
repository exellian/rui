use std::sync::Arc;
use crate::alloc::spmc::concurrent_queue::ConcurrentQueue;

#[derive(Clone)]
pub struct Receiver<T> {
    queue: Arc<ConcurrentQueue<T>>
}

unsafe impl<T> Send for Receiver<T> {}
unsafe impl<T> Sync for Receiver<T> {}

impl<T> Receiver<T> {
    pub(crate) fn new(queue: Arc<ConcurrentQueue<T>>) -> Self {
        Receiver {
            queue
        }
    }

    pub fn try_recv(&self) -> Option<T> {
        self.queue.try_pop()
    }
}