use crate::Backend;
use crate::surface::builder::Builder;
use crate::surface::surface_factory::SurfaceFactory;

pub struct Surface<B>(<B::SurfaceFactory as SurfaceFactory<B::UserEvent>>::Surface) where B: Backend;

impl<B> Surface<B> where B: Backend {
    pub fn new(surface: <B::SurfaceFactory as SurfaceFactory<B::UserEvent>>::Surface) -> Self {
        Surface(surface)
    }
    pub fn builder() -> Builder<B> {
        Builder::new()
    }
}