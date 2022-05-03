use crate::instance::Instance;
use crate::surface::Surface;
use crate::Backend;
use alloc::borrow::Cow;
use rui_io::surface::SurfaceBuilder;
use rui_io::OsError;
use rui_util::Extent;

pub struct Builder<'a> {
    inner: SurfaceBuilder<'a>,
}
impl<'a> Builder<'a> {
    pub fn new() -> Self {
        Builder {
            inner: SurfaceBuilder::new(),
        }
    }

    //TODO think about renaming to with_size and also for all the other functions
    pub fn size(mut self, extent: Extent) -> Self {
        self.inner = self.inner.with_size(extent);
        self
    }

    pub fn title(mut self, title: impl Into<Cow<'a, str>>) -> Self {
        self.inner = self.inner.with_title(title);
        self
    }

    //TODO add more properties to builder

    pub fn build<B>(self) -> Result<Surface, OsError>
    where
        B: Backend,
    {
        todo!()
    }
}
