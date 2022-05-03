mod receiver;
mod sender;

use crate::alloc::mpsc::receiver::Receiver;
use crate::alloc::mpsc::sender::Sender;
use crossbeam::channel;

pub fn unbounded<T>() -> (Sender<T>, Receiver<T>) {
    let (s, r) = channel::unbounded();
    (Sender::new(s), Receiver::new(r))
}
