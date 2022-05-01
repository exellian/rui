use crate::event::Event;

pub trait Queue {

    fn push(&mut self, event: Event);
    fn try_pop(&mut self) -> Option<Event>;
}