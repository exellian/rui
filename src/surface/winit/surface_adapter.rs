use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};
use raw_window_handle::{HasRawWindowHandle, RawWindowHandle};
use winit::window::WindowId;
use crate::surface::SurfaceId;
use crate::util::Extent;

pub struct SurfaceAdapter(winit::window::Window);
impl SurfaceAdapter {
    pub fn new(window: winit::window::Window) -> Self {
        SurfaceAdapter(window)
    }
}
impl crate::surface::SurfaceAdapter for SurfaceAdapter {

    fn inner_size(&self) -> Extent {
        let size = self.0.inner_size();
        Extent {
            width: size.width,
            height: size.height
        }
    }

    fn id(&self) -> SurfaceId {
        self.0.id().into()
    }

    fn request_redraw(&self) {
        self.0.request_redraw();
    }
}
impl From<WindowId> for SurfaceId {
    fn from(id: WindowId) -> Self {
        let mut hasher = DefaultHasher::new();
        id.hash(&mut hasher);
        SurfaceId::from(hasher.finish())
    }
}
unsafe impl HasRawWindowHandle for SurfaceAdapter {
    fn raw_window_handle(&self) -> RawWindowHandle {
        self.0.raw_window_handle()
    }
}