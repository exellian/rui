mod builder;

use crate::Node;
pub use builder::Builder as SurfaceBuilder;
use raw_window_handle::{HasRawWindowHandle, RawWindowHandle};
use rui_io::surface::SurfaceId;
use rui_util::Extent;

pub struct Surface(SurfaceId);

impl Surface {
    fn new(surface: SurfaceId) -> Self {
        Surface(surface)
    }

    pub fn builder<'a>() -> SurfaceBuilder<'a> {
        SurfaceBuilder::new()
    }

    pub async fn mount(&self, node: Node) {
        todo!()
    }

    pub fn inner_size(&self) -> Extent {
        todo!()
    }

    pub fn id(&self) -> SurfaceId {
        self.0
    }

    pub fn request_redraw(&self) {
        todo!()
    }
}

unsafe impl HasRawWindowHandle for Surface {
    fn raw_window_handle(&self) -> RawWindowHandle {
        todo!()
    }
}
