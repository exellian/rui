use std::fmt::{Debug, Formatter};
use std::marker::PhantomData;
use crate::event::EventLoop;
use crate::instance;

#[derive(Debug)]
pub struct WGpu<T=()>(PhantomData<T>);
impl<T> super::Backend for WGpu<T> where T: 'static + Sync + Send + Debug {
    type UserEvent = T;
    type Surface = crate::surface::winit::SurfaceAdapter;
    type SurfaceFactory = crate::surface::winit::SurfaceFactory<T>;
    type EventLoopTarget = <Self::EventLoop as EventLoop<T>>::EventLoopTarget;
    type EventLoop = crate::event::winit::EventLoop<T>;
    type Renderer = crate::renderer::wgpu::Renderer<Self>;
}