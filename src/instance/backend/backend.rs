use std::fmt::Debug;
use std::hash::Hash;
use crate::event::EventLoop;
use crate::renderer::Renderer;
use crate::surface::{Surface, SurfaceAdapter, SurfaceFactory};

pub trait Backend: Sync + Send {
    /// Should be an enum type that
    /// allows the user to define own events
    type UserEvent: 'static + Debug;

    /// Surface type that should be used to create e.g: windows
    /// Default is the winit library
    type Surface: SurfaceAdapter;

    /// A factory for creating surfaces.
    /// This is needed because the user should use the surface::Surface struct
    /// to create
    type SurfaceFactory: SurfaceFactory<Self::UserEvent, Surface=Self::Surface> + Hash;

    /// The event loop that should be used. Hint: it must be compatible 
    /// with the surface
    type EventLoop: EventLoop<Self::UserEvent, Surface=Self::Surface>;
    
    /// The renderer to use that supports the given surface
    type Renderer: Renderer<Self::UserEvent>;

}