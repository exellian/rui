use std::convert::Infallible;
use std::fmt::Debug;
use crate::surface::{SurfaceEvent, SurfaceId};
use crate::util::Handler;

#[derive(Debug)]
pub enum Event<T> {
    UserEvent(T),
    SurfaceEvent {
        id: SurfaceId,
        event: SurfaceEvent
    },
    EventsCleared
}
impl<F, T> Handler<Event<T>> for F where T: Debug, F: FnMut(Event<T>) {
    type Error = Infallible;

    fn handle(&mut self, event: Event<T>) -> Result<(), Self::Error> {
        Ok(self(event))
    }
}
