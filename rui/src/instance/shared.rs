use crate::error::Error;
use rui_io::surface::{SurfaceAttributes, SurfaceId};
use rui_util::alloc::{mpsc, oneshot};
use std::sync::{Arc, RwLock};

use crate::instance::main_loop_request::MainLoopRequest;
use crate::instance::InstanceShared;
use crate::reactor::Reactor;
use crate::surface::SurfaceSharedState;
use crate::Node;

pub struct Shared {
    main_loop_sender: mpsc::Sender<MainLoopRequest>,
}
impl Shared {
    pub(crate) fn new(main_loop_sender: mpsc::Sender<MainLoopRequest>) -> Self {
        InstanceShared { main_loop_sender }
    }

    pub(crate) async fn mount(&self, surface_id: SurfaceId, node: Node) -> Result<(), Error> {
        let (sender, mut receiver) = oneshot::channel();
        self.main_loop_sender.send(MainLoopRequest::MountNode {
            surface_id,
            node,
            sender,
        });
        receiver.recv().await
    }

    pub(crate) async fn create_surface(
        &self,
        attr: SurfaceAttributes,
    ) -> Arc<RwLock<SurfaceSharedState>> {
        let (sender, mut receiver) = oneshot::channel();
        Reactor::get()
            .shared
            .main_loop_sender
            .send(MainLoopRequest::CreateSurface { attr, sender });
        receiver.recv().await
    }
}
