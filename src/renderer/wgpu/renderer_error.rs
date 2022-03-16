use std::fmt::{Debug, Display, Formatter};

pub enum RendererError {
    AdapterNotFound,
    DeviceCreationFailed(wgpu::RequestDeviceError),
}

impl Debug for RendererError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        todo!()
    }
}

impl Display for RendererError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        todo!()
    }
}

impl std::error::Error for RendererError {}