use std::fmt::{Debug, Display, Formatter};

pub enum RendererError {
    AdapterNotFound,
    DeviceCreationFailed(wgpu::RequestDeviceError),
}

impl Debug for RendererError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            RendererError::AdapterNotFound => {
                write!(f, "Adapter not found!")
            }
            RendererError::DeviceCreationFailed(err) => {
                Debug::fmt(err, f)
            }
        }
    }
}

impl Display for RendererError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        Debug::fmt(self, f)
    }
}

impl std::error::Error for RendererError {}