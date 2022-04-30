mod sender;
mod receiver;

use crossbeam::channel;
use crate::alloc::mpsc::receiver::Receiver;
use crate::alloc::mpsc::sender::Sender;

pub fn unbounded<T>() -> (Sender<T>, Receiver<T>) {
    let (s, r) = channel::unbounded();
    (Sender::new(s), Receiver::new(r))
}
