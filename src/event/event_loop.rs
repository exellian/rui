use crate::event::event::Event;
use crate::util::Handler;

pub trait EventLoop<T> {
    type Surface;

    fn run<H>(self, handler: H) where H: Handler<Event<T>>;
}