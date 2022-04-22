use std::future::Future;
use std::pin::Pin;
use std::task::{Context, Poll};
use crate::event::Event;
use crate::alloc::mpmc;

pub struct EventFuture<'a> {
    receiver: &'a mpmc::Receiver<Event>
}
impl<'a> EventFuture<'a> {

    pub fn new(receiver: &'a mpmc::Receiver<Event>) -> Self {
        EventFuture {
            receiver
        }
    }
}
impl<'a> Future for EventFuture<'a> {
    type Output = Event;

    fn poll(self: Pin<&mut Self>, _: &mut Context<'_>) -> Poll<Self::Output> {
        match self.receiver.try_recv() {
            Some(event) => Poll::Ready(event),
            None => Poll::Pending
        }
    }
}