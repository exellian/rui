use std::error::Error;

pub trait SurfaceFactory {
    type Surface;
    type Error: Error;

    fn create() -> Result<Self::Surface, Self::Error>;
}