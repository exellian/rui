use crate::Backend;
use crate::instance::error::Error;
use crate::surface::{SurfaceAttributes, SurfaceFactory};

pub enum MainThreadRequest {
    CreateSurface(SurfaceAttributes)
}
pub enum MainThreadResponse<B> where B: Backend {
    Surface(Result<B::Surface, Error<B>>)
}