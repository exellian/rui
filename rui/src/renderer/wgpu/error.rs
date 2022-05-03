use std::fmt::{Debug, Display, Formatter};

pub enum Error {
    AdapterNotFound,
    DeviceCreationFailed(wgpu::RequestDeviceError),
}

impl Debug for Error {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Error::AdapterNotFound => {
                write!(f, "Adapter not found!")
            }
            Error::DeviceCreationFailed(err) => Debug::fmt(err, f),
        }
    }
}

impl Display for Error {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        Debug::fmt(self, f)
    }
}

impl std::error::Error for Error {}
