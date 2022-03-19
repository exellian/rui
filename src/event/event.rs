use std::convert::Infallible;
use std::fmt::Debug;
use crate::util::Handler;

#[derive(Debug)]
pub enum Event<T> {
    UserEvent(T)
}
impl<F, T> Handler<Event<T>> for F where T: Debug, F: FnMut(Event<T>) {
    type Error = Infallible;

    fn handle(&mut self, event: Event<T>) -> Result<(), Self::Error> {
        Ok(self(event))
    }
}
