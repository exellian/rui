mod attributes;
mod error;
mod event;
pub mod id;

pub use attributes::Attributes as SurfaceAttributes;
pub use attributes::Modality;
pub use attributes::WindowState;
pub use error::Error as SurfaceError;
pub use event::Event as SurfaceEvent;
pub use id::Id as SurfaceId;
use raw_window_handle::{HasRawWindowHandle, RawWindowHandle};
use rui_util::Extent;

use crate::platform;

pub struct Surface<'main, 'child>(platform::Surface<'main, 'child>);

impl<'main, 'child> Surface<'main, 'child> {
    pub fn new(surface: platform::Surface<'main, 'child>) -> Self {
        Surface(surface)
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
