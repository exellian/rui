use std::fmt::Debug;

#[derive(Debug)]
pub struct WGpu;
impl super::Backend for WGpu {
    type Renderer = crate::renderer::wgpu::Renderer<Self>;
}