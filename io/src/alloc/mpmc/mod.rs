mod receiver;
mod sender;

use std::sync::Arc;
use crossbeam::queue;
pub use receiver::Receiver;
pub use sender::Sender;

pub fn unbounded<T>() -> (Sender<T>, Receiver<T>) {
    let queue = Arc::new(queue::SegQueue::new());
    (Sender::new(queue.clone()), Receiver::new(queue))
}
