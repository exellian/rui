use std::fmt::Debug;
use tokio::sync::mpsc;
use crate::util::Handler;

pub struct Sender<T>(mpsc::UnboundedSender<T>);
impl<T> Sender<T> {
    pub fn new(sender: mpsc::UnboundedSender<T>) -> Self {
        Sender(sender)
    }
}
impl<T> Handler<T> for Sender<T> where T: Debug {
    type Error = mpsc::error::SendError<T>;

    fn handle(&mut self, event: T) -> Result<(), Self::Error> {
        self.0.send(event)
    }
}