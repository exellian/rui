mod child_loop;
mod exit_code;
mod flow;
pub(crate) mod inner;
mod loop_control;
mod loop_state;
mod loop_target;
mod main_loop;
pub(crate) mod queue;

use crate::surface::{SurfaceEvent, SurfaceId};

pub use child_loop::ChildLoop as ChildEventLoop;
pub use flow::Flow;
pub use loop_target::LoopTarget as EventLoopTarget;
pub use main_loop::MainLoop as MainEventLoop;

pub use loop_target::LoopTarget;

#[derive(Clone, Debug)]
pub enum Event {
    Init,
    SurfaceEvent { id: SurfaceId, event: SurfaceEvent },
    EventsCleared,
    Default,
}
