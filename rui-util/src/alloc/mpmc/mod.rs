mod receiver;
mod sender;

use crossbeam::queue;
pub use receiver::Receiver;
pub use sender::Sender;
use std::sync::Arc;

pub fn unbounded<T>() -> (Sender<T>, Receiver<T>) {
    let queue = Arc::new(queue::SegQueue::new());
    (Sender::new(queue.clone()), Receiver::new(queue))
}
