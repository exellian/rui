use std::fmt::Debug;
use winit::event_loop::ControlFlow;
use crate::event::event::Event;
use crate::surface;
use crate::util::Handler;

pub struct EventLoop<T>(pub(crate) winit::event_loop::EventLoop<T>) where T: 'static;

impl<T> Default for EventLoop<T> where T: 'static {
    fn default() -> Self {
        EventLoop(winit::event_loop::EventLoop::with_user_event())
    }
}
impl<T> super::super::EventLoop<T> for EventLoop<T> where T: Debug {
    type Surface = surface::winit::Surface;

    fn run<H>(self, mut handler: H) where H: Handler<Event<T>> + 'static {
        self.0.run(move |winit_event, l, control_flow| {
            *control_flow = ControlFlow::Wait;
            let event = match winit_event.try_into() {
                Ok(e) => e,
                Err(_) => return
            };
            if let Err(err) = handler.handle(event) {
                eprintln!("{}", err)
            }
        })
    }
}