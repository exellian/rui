use crate::alloc::oneshot::channel::Channel;
use std::future::Future;
use std::pin::Pin;
use std::sync::Arc;
use std::task::{Context, Poll};

pub struct Recv<'a, T> {
    receiver: &'a mut Receiver<T>,
}

impl<'a, T> Future for Recv<'a, T> {
    type Output = T;

    fn poll(mut self: Pin<&mut Self>, _: &mut Context<'_>) -> Poll<Self::Output> {
        match unsafe { self.receiver.inner.try_recv() } {
            None => Poll::Pending,
            Some(val) => {
                self.receiver.taken = true;
                Poll::Ready(val)
            }
        }
    }
}

pub struct Receiver<T> {
    inner: Arc<Channel<T>>,
    taken: bool,
}
impl<T> Receiver<T> {
    pub(crate) fn new(inner: Arc<Channel<T>>) -> Self {
        Receiver {
            inner,
            taken: false,
        }
    }

    pub fn try_recv(&mut self) -> Option<T> {
        match self.taken {
            true => panic!("Value already got taken!"),
            false => match unsafe { self.inner.try_recv() } {
                None => None,
                Some(x) => {
                    self.taken = true;
                    Some(x)
                }
            },
        }
    }

    pub fn recv(&mut self) -> Recv<T> {
        match self.taken {
            true => panic!("Value already got taken!"),
            false => Recv { receiver: self },
        }
    }
}
