use std::sync::Arc;

use crate::alloc::oneshot::channel::Channel;
pub use crate::alloc::oneshot::receiver::Receiver;
pub use crate::alloc::oneshot::sender::Sender;

mod channel;
mod receiver;
mod sender;

pub fn channel<T>() -> (Sender<T>, Receiver<T>) {
    let channel = Arc::new(Channel::new());
    (Sender::new(channel.clone()), Receiver::new(channel))
}
