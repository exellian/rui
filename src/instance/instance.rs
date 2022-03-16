use crate::event::EventLoop;
use crate::renderer::Renderer;
use crate::state::State;
use crate::surface::{Surface, SurfaceFactory};

pub struct Instance<E, S, SF, R> where
    E: EventLoop<Surface=S>,
    S: Surface<EventLoop=E>,
    SF: SurfaceFactory<Surface=S>,
    R: Renderer<S>
{
    event_loop: E,
    surface_factory: SF,
    renderer: R
}
impl<E, S, SF, R> Instance<E, S, SF, R> where
    E: EventLoop<Surface=S>,
    S: Surface<EventLoop=E>,
    SF: SurfaceFactory<Surface=S>,
    R: Renderer<S>
{

    pub fn new(event_loop: E, surface_factory: SF, renderer: R) -> Self {
        Instance {
            event_loop,
            surface_factory,
            renderer
        }
    }

    pub async fn run<T>(self, state: T) where T: State {

    }
}