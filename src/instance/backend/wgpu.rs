use std::fmt::Debug;
use std::marker::PhantomData;

pub struct WGpu<T>(PhantomData<T>);
impl<T> super::Backend for WGpu<T> where T: 'static + Sync + Send + Debug {
    type UserEvent = T;
    type Surface = crate::surface::winit::Surface;
    type EventLoop = crate::event::winit::EventLoop<T>;
    type Renderer = crate::renderer::wgpu::Renderer;
}