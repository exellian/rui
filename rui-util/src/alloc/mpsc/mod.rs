mod receiver;
mod sender;

pub use crate::alloc::mpsc::receiver::Receiver;
pub use crate::alloc::mpsc::sender::Sender;
use crossbeam::channel;

pub fn unbounded<T>() -> (Sender<T>, Receiver<T>) {
    let (s, r) = channel::unbounded();
    (Sender::new(s), Receiver::new(r))
}
