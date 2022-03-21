use std::fmt::Debug;
use std::ops::Deref;
use winit::event_loop::ControlFlow;
use crate::event::event::Event;

pub struct EventLoop<T>(pub(crate) winit::event_loop::EventLoop<T>) where T: 'static;

impl<T> Default for EventLoop<T> where T: 'static {
    fn default() -> Self {
        EventLoop(winit::event_loop::EventLoop::with_user_event())
    }
}
impl<T> super::super::EventLoop<T> for EventLoop<T> where T: Debug {
    type EventLoopTarget = winit::event_loop::EventLoopWindowTarget<T>;

    fn run<F>(self, mut handler: F) where F: FnMut(Event<T>, &Self::EventLoopTarget) + 'static {
        self.0.run(move |winit_event, l, control_flow| {
            *control_flow = ControlFlow::Wait;
            let event = match winit_event.try_into() {
                Ok(e) => e,
                Err(_) => return
            };
            handler(event, l);
        })
    }
}
impl<T> Deref for EventLoop<T> {
    type Target = winit::event_loop::EventLoopWindowTarget<T>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}