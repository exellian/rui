use util::Extent;
use crate::os_error::OsError;
use crate::runtime::Runtime;
use crate::surface::Surface;
use crate::surface::SurfaceAttributes;
use std::borrow::Cow;

pub struct Builder<'a> {
    title: Option<Cow<'a, str>>,
    size: Option<Extent>
}
impl<'a> Builder<'a> {

    pub fn new() -> Self {
        Builder {
            title: None,
            size: None
        }
    }

    pub fn size(mut self, extent: Extent) -> Self {
        self.size = Some(extent);
        self
    }

    pub fn title(mut self, title: impl Into<Cow<'a, str>>) -> Self {
        self.title = Some(title.into());
        self
    }
    
    pub fn build(self, runtime: &Runtime) -> Result<Surface, OsError> {
        let attributes = SurfaceAttributes::new(self.title.unwrap(), self.size.unwrap());
        Surface::try_from(&attributes)
    }
}