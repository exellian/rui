mod event;
mod builder;
mod attributes;
pub mod id;
mod error;

use raw_window_handle::{HasRawWindowHandle, RawWindowHandle};
pub use id::Id as SurfaceId;
pub use builder::Builder as SurfaceBuilder;
pub use attributes::Attributes as SurfaceAttributes;
pub use event::Event as SurfaceEvent;
pub use error::Error as SurfaceError;
use rui_util::Extent;

use crate::platform;

pub struct Surface<'main, 'child>(platform::Surface<'main, 'child>);

impl<'main, 'child> Surface<'main, 'child> {

    fn new(surface: platform::Surface<'main, 'child>) -> Self {
        Surface(surface)
    }

    pub fn builder<'a>() -> SurfaceBuilder<'a> {
        SurfaceBuilder::new()
    }

    pub fn inner_size(&self) -> Extent {
        todo!()
    }

    pub fn id(&self) -> SurfaceId {
        todo!()
    }

    pub fn request_redraw(&self) {
        todo!()
    }
}

unsafe impl<'main, 'child> HasRawWindowHandle for Surface<'main, 'child> {
    fn raw_window_handle(&self) -> RawWindowHandle {
        self.0.raw_window_handle()
    }
}
