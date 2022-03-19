use winit::dpi::PhysicalSize;
use crate::event::winit::EventLoop;
use crate::surface::surface_attributes::SurfaceAttributes;

pub struct SurfaceFactory;

impl<T> crate::surface::SurfaceFactory<T> for SurfaceFactory {
    type EventLoop = EventLoop<T>;
    type Error = winit::error::OsError;
    type Surface = winit::window::Window;

    fn build(event_loop: &Self::EventLoop, attributes: SurfaceAttributes) -> Result<Self::Surface, Self::Error> {
        //TODO add support for other surface attributes
        winit::window::WindowBuilder::new()
            .with_inner_size(PhysicalSize {
                width: attributes.size.width,
                height: attributes.size.height
            })
            .with_title(attributes.title)
            .build(&event_loop.0)
    }
}