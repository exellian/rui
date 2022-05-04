use std::sync::{Arc, RwLock};

use crate::error::Error;
use rui_io::surface::{SurfaceAttributes, SurfaceId};
use rui_util::alloc::oneshot;

use crate::surface::SurfaceSharedState;
use crate::Node;

pub enum MainLoopRequest {
    CreateSurface {
        attr: SurfaceAttributes,
        sender: oneshot::Sender<Arc<RwLock<SurfaceSharedState>>>,
    },
    MountNode {
        surface_id: SurfaceId,
        node: Node,
        sender: oneshot::Sender<Result<(), Error>>,
    },
}
