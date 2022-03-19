use std::hash::{Hash, Hasher};
use std::sync::atomic::{AtomicU64, Ordering};
use raw_window_handle::{HasRawWindowHandle, RawWindowHandle};
use crate::util::Extent;

pub struct SurfaceAdapter(u64, winit::window::Window);
impl SurfaceAdapter {
    pub fn new<T>(window: winit::window::Window) -> Self {
        // Ids for hashing:
        // Every window instance should have an unique id
        // to be able to be hashed (see: impl Hash for SurfaceAdapter)
        static IDS: AtomicU64 = AtomicU64::new(0);
        let id = IDS.fetch_add(1, Ordering::Acquire);
        SurfaceAdapter(id, window)
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
}
impl Hash for SurfaceAdapter {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.0.hash(state)
    }
}
unsafe impl HasRawWindowHandle for SurfaceAdapter {
    fn raw_window_handle(&self) -> RawWindowHandle {
        self.0.raw_window_handle()
    }
}