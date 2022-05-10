mod exit_code;
mod flow;
mod inner_loop;
mod r#loop;
mod loop_control;
mod loop_state;
mod loop_target;
mod main_loop;
pub(crate) mod queue;

use crate::surface::{SurfaceEvent, SurfaceId};

pub use flow::Flow;
pub(crate) use inner_loop::InnerLoop;
pub use loop_target::LoopTarget as EventLoopTarget;
pub use main_loop::MainLoop as MainEventLoop;
pub use r#loop::Loop as EventLoop;

pub use loop_target::LoopTarget;

#[derive(Clone, Debug)]
pub enum Event {
    Init,
    SurfaceEvent { id: SurfaceId, event: SurfaceEvent },
    EventsCleared,
    Default,
}
