mod builder;

use raw_window_handle::{HasRawWindowHandle, RawWindowHandle};
pub use builder::Builder as SurfaceBuilder;
use io::surface::SurfaceId;
use util::Extent;

pub struct Surface(io::surface::Surface);

impl Surface {

    pub fn new(surface: io::surface::Surface) -> Self {
        Surface(surface)
    }

    pub fn builder<'a>() -> SurfaceBuilder<'a> {
        SurfaceBuilder::new()
    }

    pub fn inner_size(&self) -> Extent {
        self.0.inner_size()
    }

    pub fn id(&self) -> SurfaceId {
        self.0.id()
    }

    pub fn request_redraw(&self) {
        self.0.request_redraw()
    }
}

unsafe impl HasRawWindowHandle for Surface {
    fn raw_window_handle(&self) -> RawWindowHandle {
        self.0.raw_window_handle()
    }
}