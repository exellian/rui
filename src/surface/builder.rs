use crate::Backend;
use crate::instance::Instance;
use crate::surface::surface_attributes::SurfaceAttributes;
use crate::surface::surface_factory::SurfaceFactory;
use crate::util::Extent;

pub struct Builder<B> where B: Backend {
    size: Option<Extent>,
    title: Option<String>,
}
impl<B> Builder<B> where B: Backend {
    pub fn new() -> Self {
        Builder {
            size: None,
            title: None
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

    pub fn build(self, instance: &Instance<B>) -> Result<
        B::Surface, 
        <B::SurfaceFactory as SurfaceFactory<B::UserEvent>>::Error
    > {
        let attributes = SurfaceAttributes {
            size: self.size.unwrap(),
            title: self.title.unwrap()
        };
        
    }
}