use raw_window_handle::RawWindowHandle;

use rui_io::surface::{SurfaceAttributes, SurfaceId};

pub struct State {
    pub(crate) id: SurfaceId,
    pub(crate) attr: SurfaceAttributes,
    pub(crate) raw_handle: RawWindowHandle,
}
unsafe impl Send for State {}
unsafe impl Sync for State {}
impl State {
    pub fn new(id: SurfaceId, attr: SurfaceAttributes, raw_handle: RawWindowHandle) -> Self {
        State {
            id,
            attr,
            raw_handle,
        }
    }
}
