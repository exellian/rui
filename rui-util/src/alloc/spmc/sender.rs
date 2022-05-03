use crate::alloc::spmc::concurrent_queue::ConcurrentQueue;
use std::sync::Arc;

pub struct Sender<T> {
    queue: Arc<ConcurrentQueue<T>>,
}

unsafe impl<T> Send for Sender<T> {}
unsafe impl<T> Sync for Sender<T> {}

impl<T> Sender<T> {
    pub fn new(queue: Arc<ConcurrentQueue<T>>) -> Self {
        Sender { queue }
    }

    pub fn send(&mut self, mut x: T) {
        while let Err(_x) = unsafe { self.queue.try_push(x) } {
            x = _x;
        }
    }

    pub fn try_send(&mut self, x: T) -> Result<(), T> {
        unsafe { self.queue.try_push(x) }
    }
}
