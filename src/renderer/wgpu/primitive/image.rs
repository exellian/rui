use crate::renderer::wgpu::pipeline::image_pipeline;

pub struct Image {
    pub(crate) instance: image_pipeline::Instance,
    pub(crate) resource: String
}