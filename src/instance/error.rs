use std::fmt;
use std::fmt::{Display, Formatter};
use crate::Backend;
use crate::renderer::Renderer;
use crate::surface::SurfaceFactory;

pub enum Error<B> where B: Backend {
    SurfaceError(<B::SurfaceFactory as SurfaceFactory>::Error),
    RendererError(<B::Renderer as Renderer<B>>::Error)
}
impl<B> fmt::Debug for Error<B> where B: Backend {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Error::SurfaceError(e) => fmt::Debug::fmt(e, f),
            Error::RendererError(e) => fmt::Debug::fmt(e, f)
        }
    }
}