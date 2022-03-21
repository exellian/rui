use crate::Backend;
use crate::surface::builder::Builder;
use crate::surface::surface_factory::SurfaceFactory;

pub struct Surface<B>(<B::SurfaceFactory as SurfaceFactory>::Surface) where B: Backend;

impl<B> Surface<B> where B: Backend + 'static {
    pub fn new(surface: <B::SurfaceFactory as SurfaceFactory>::Surface) -> Self {
        Surface(surface)
    }
    pub fn builder() -> Builder<B> {
        Builder::new()
    }
}