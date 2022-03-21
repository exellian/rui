use std::marker::PhantomData;
use crate::Backend;
use crate::instance::{Instance, WGpu};
use crate::surface::surface_attributes::SurfaceAttributes;
use crate::util::Extent;
use crate::instance;
use crate::surface::SurfaceFactory;

pub struct Builder<B=WGpu> where B: Backend {
    size: Option<Extent>,
    title: Option<String>,
    _b: PhantomData<B>
}
impl<B> Builder<B> where B: Backend + 'static {
    pub fn new() -> Self {
        Builder {
            size: None,
            title: None,
            _b: PhantomData
        }
    }

    //TODO think about renaming to with_size and also for all the other functions
    pub fn size(mut self, extent: Extent) -> Self {
        self.size = Some(extent);
        self
    }

    pub fn title<S>(mut self, title: S) -> Self where S: Into<String> {
        self.title = Some(title.into());
        self
    }

    //TODO add more properties to builder

    pub fn build(self, instance: &Instance<B>) -> Result<B::Surface, <B::SurfaceFactory as SurfaceFactory>::Error> {
        let attributes = SurfaceAttributes {
            size: self.size.unwrap(),
            title: self.title.unwrap()
        };
        instance.create_surface(attributes)
    }
}