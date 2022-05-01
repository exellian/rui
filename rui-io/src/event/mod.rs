mod flow;
mod main_loop;
mod r#loop;
mod loop_state;
mod loop_control;
mod loop_target;
mod inner_loop;
mod exit_code;
mod queue;

use crate::surface::{SurfaceEvent, SurfaceId};

pub use flow::Flow;
pub(crate) use inner_loop::InnerLoop;
pub(crate) use queue::Queue;

pub use loop_target::LoopTarget;


#[derive(Clone)]
pub enum Event {
    SurfaceEvent {
        id: SurfaceId,
        event: SurfaceEvent
    },
    EventsCleared,
    Default
}