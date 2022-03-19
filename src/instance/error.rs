use crate::Backend;
use crate::renderer::Renderer;
use crate::surface::SurfaceFactory;

pub enum Error<B> where B: Backend {
    SurfaceError(<B::SurfaceFactory as SurfaceFactory<B::UserEvent>>::Error),
    RendererError(<B::Renderer as Renderer>::Error)
}