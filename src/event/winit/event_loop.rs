use winit::event_loop::ControlFlow;
use crate::event::event::Event;
use crate::surface;
use crate::util::Handler;

pub struct EventLoop<T>(winit::event_loop::EventLoop<T>) where T: 'static;

impl<T> super::super::EventLoop<T> for EventLoop<T> {
    type Surface = surface::winit::Surface;

    fn run<H>(self, mut handler: H) where H: Handler<Event<T>> + 'static {
        self.0.run(move |winit_event, l, control_flow| {
            *control_flow = ControlFlow::Wait;
            let event = match winit_event.try_into() {
                Ok(e) => e,
                Err(_) => return
            };
            handler.handle(event);
        })
    }
}