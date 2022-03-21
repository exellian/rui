use std::fmt::Debug;
use std::marker::PhantomData;
use winit::dpi::PhysicalSize;
use crate::event::winit::EventLoop;
use crate::surface::surface_attributes::SurfaceAttributes;
use crate::surface::winit::SurfaceAdapter;

pub struct SurfaceFactory<T>(PhantomData<T>);

impl<T> crate::surface::SurfaceFactory for SurfaceFactory<T> where T: 'static + Debug {
    type EventLoopTarget = winit::event_loop::EventLoopWindowTarget<T>;
    type Error = winit::error::OsError;
    type Surface = SurfaceAdapter;

    fn build(event_loop: &Self::EventLoopTarget, attributes: SurfaceAttributes) -> Result<Self::Surface, Self::Error> {
        //TODO add support for other surface attributes
        let window = winit::window::WindowBuilder::new()
            .with_inner_size(PhysicalSize {
                width: attributes.size.width,
                height: attributes.size.height
            })
            .with_title(attributes.title)
            .build(event_loop)?;
        Ok(SurfaceAdapter::new(window))
    }
}