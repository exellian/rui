use crate::alloc::spmc::concurrent_queue::ConcurrentQueue;
use std::sync::Arc;

#[derive(Clone)]
pub struct Receiver<T> {
    queue: Arc<ConcurrentQueue<T>>,
}

unsafe impl<T> Send for Receiver<T> {}
unsafe impl<T> Sync for Receiver<T> {}

impl<T> Receiver<T> {
    pub fn new(queue: Arc<ConcurrentQueue<T>>) -> Self {
        Receiver { queue }
    }

    pub fn try_recv(&self) -> Option<T> {
        self.queue.try_pop()
    }
}
