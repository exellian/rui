use std::fmt;
use std::fmt::Formatter;
use io::surface::SurfaceError;
use crate::Backend;
use crate::renderer::Renderer;

pub enum Error<B> where B: Backend {
    SurfaceError(SurfaceError),
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