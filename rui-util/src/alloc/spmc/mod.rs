mod concurrent_queue;
mod receiver;
mod sender;

pub use receiver::Receiver;
pub use sender::Sender;
use std::sync::Arc;

pub fn channel<T>(size: usize) -> (Sender<T>, Receiver<T>) {
    let queue = Arc::new(concurrent_queue::ConcurrentQueue::new(size));
    (Sender::new(queue.clone()), Receiver::new(queue.clone()))
}
