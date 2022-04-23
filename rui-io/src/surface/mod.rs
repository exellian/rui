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

use crate::os_error::OsError;
use crate::platform;

pub struct Surface(platform::Surface);

impl Surface {

    fn new(surface: platform::Surface) -> Self {
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

impl<'a> TryFrom<&SurfaceAttributes<'a>> for Surface {
    type Error = OsError;

    fn try_from(value: &SurfaceAttributes<'a>) -> Result<Self, Self::Error> {
        let surface = platform::Surface::try_from(value)?;
        Ok(Surface::new(surface))
    }
}

unsafe impl HasRawWindowHandle for Surface {
    fn raw_window_handle(&self) -> RawWindowHandle {
        self.0.raw_window_handle()
    }
}
