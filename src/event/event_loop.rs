use std::fmt::Debug;
use crate::event::event::Event;
use crate::util::Handler;

pub trait EventLoop<T> where T: Debug {
    type Surface;

    fn run<H>(self, handler: H) where H: Handler<Event<T>> + 'static;
}