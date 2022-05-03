use crate::renderer::wgpu::primitive;
use crate::util::Resource;

pub struct Image {
    pub(crate) instance: primitive::Instance,
    pub(crate) resource: Resource,
}
