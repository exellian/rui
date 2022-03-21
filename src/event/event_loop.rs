use std::fmt::Debug;
use crate::event::event::Event;

pub trait EventLoop<T> where T: Debug {

    type EventLoopTarget;

    fn run<F>(self, handler: F) where F: FnMut(Event<T>, &Self::EventLoopTarget) + 'static;
}