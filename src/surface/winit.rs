use winit::event_loop::EventLoop;
use crate::surface::surface_event::SurfaceEvent;
use crate::event;

pub struct Surface {

}
impl Surface {

}
impl super::Surface for Surface {
    type EventLoop = event::winit::EventLoop<()>;
    type Error = winit::error::ExternalError;
}