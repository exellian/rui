use std::sync::{Arc, RwLock};

use raw_window_handle::{HasRawWindowHandle, RawWindowHandle};

use crate::error::Error;
pub use builder::Builder as SurfaceBuilder;
use rui_io::surface::{SurfaceAttributes, SurfaceId};
use rui_util::Extent;
pub(crate) use shared::SharedState as SurfaceSharedState;

use crate::reactor::Reactor;
use crate::Node;

mod builder;
mod shared;

pub struct Surface {
    shared_state: Arc<RwLock<SurfaceSharedState>>,
}

impl Surface {
    async fn new(attr: SurfaceAttributes) -> Self {
        let shared_state = Reactor::get().shared.create_surface(attr).await;
        Surface { shared_state }
    }

    pub fn builder() -> SurfaceBuilder {
        SurfaceBuilder::new()
    }

    pub async fn mount(&self, node: Node) -> Result<(), Error> {
        Reactor::get().shared.mount(self.id(), node).await
    }

    pub fn inner_size(&self) -> Extent {
        todo!()
    }

    pub fn id(&self) -> SurfaceId {
        self.shared_state.read().unwrap().id
    }

    pub fn request_redraw(&self) {
        todo!()
    }

    pub fn raw_handle(&self) -> RawWindowHandle {
        self.shared_state.read().unwrap().raw_handle
    }
}
