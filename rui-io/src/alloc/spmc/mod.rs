mod sender;
mod receiver;
mod concurrent_queue;

use std::sync::Arc;
pub use sender::Sender;
pub use receiver::Receiver;

pub fn channel<T>(size: usize) -> (Sender<T>, Receiver<T>) {
    let queue = Arc::new(concurrent_queue::ConcurrentQueue::new(size));
    (Sender::new(queue.clone()), Receiver::new(queue.clone()))
}