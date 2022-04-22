use std::fmt::{Debug, Formatter};
use std::marker::PhantomData;
use crate::instance;

#[derive(Debug)]
pub struct WGpu<T=()>(PhantomData<T>);
impl<T> super::Backend for WGpu<T> where T: 'static + Sync + Send + Debug {
    type Renderer = crate::renderer::wgpu::Renderer<Self>;
}