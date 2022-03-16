use crate::util::Handler;

pub enum Event<T> {
    UserEvent(T)
}
impl<F, T> Handler<Event<T>> for F where F: FnMut(&Event<T>) {
    type Error = ();

    fn handle(&mut self, event: &Event<T>) -> Result<(), Self::Error> {
        Ok(self(event))
    }
}
