use crate::surface::{SurfaceEvent, SurfaceId};

#[derive(Clone)]
pub enum Event {
    SurfaceEvent {
        id: SurfaceId,
        event: SurfaceEvent
    },
    EventsCleared,
    Default
}