use raw_window_handle::{HasRawWindowHandle, RawWindowHandle};

use crate::event::EventLoopTarget;
pub use attributes::Attributes as SurfaceAttributes;
pub use attributes::Modality;
pub use attributes::WindowState;
pub use error::Error as SurfaceError;
pub use event::Event as SurfaceEvent;
pub use id::Id as SurfaceId;
use rui_util::Extent;

use crate::platform;

mod attributes;
mod error;
mod event;
pub mod id;

pub struct Surface<'main, 'child>(platform::Surface<'main, 'child>);

impl<'main, 'child> Surface<'main, 'child> {
    pub fn new(loop_target: &EventLoopTarget<'main, 'child>, attr: &SurfaceAttributes) -> Self {
        Surface(platform::Surface::new(loop_target, &attr))
    }

    pub fn inner_size(&self) -> Extent {
        self.0.inner_size()
    }

    pub fn id(&self) -> SurfaceId {
        self.0.id()
    }

    pub fn request_redraw(&mut self) {
        self.0.request_redraw()
    }
}

unsafe impl<'main, 'child> HasRawWindowHandle for Surface<'main, 'child> {
    fn raw_window_handle(&self) -> RawWindowHandle {
        self.0.raw_window_handle()
    }
}
