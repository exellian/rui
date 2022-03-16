use crate::event::event::Event;
use crate::surface;
use crate::util::Handler;

pub struct EventLoop<T>(winit::event_loop::EventLoop<T>) where T: 'static;

impl<T> super::super::EventLoop for EventLoop<T> {
    type Surface = surface::winit::Surface;

    fn run<H>(self, handler: H) where H: Handler<Event<T>> {
        self.0.run(|winit_event, l, control_flow| {
            let event = winit_event.try_into();
        })
    }
}