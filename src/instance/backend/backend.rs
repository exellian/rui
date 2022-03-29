use std::fmt::Debug;
use std::hash::Hash;
use std::ops::Deref;
use crate::event::EventLoop;
use crate::renderer::Renderer;
use crate::surface::{Surface, SurfaceAdapter, SurfaceFactory};

pub trait Backend: Sync + Send + Sized {
    /// Should be an enum type that
    /// allows the user to define own events
    type UserEvent: 'static + Debug + Send;

    /// Surface type that should be used to create e.g: windows
    /// Default is the winit library
    type Surface: SurfaceAdapter + Sync;

    /// A factory for creating surfaces.
    /// This is needed because the user should use the surface::Surface struct
    /// to create
    type SurfaceFactory: SurfaceFactory<Surface=Self::Surface, EventLoopTarget=Self::EventLoopTarget>;

    /// A target type for the event loop to be compatible with the winit
    type EventLoopTarget;

    /// The event loop that should be used. Hint: it must be compatible 
    /// with the surface
    type EventLoop: EventLoop<Self::UserEvent, EventLoopTarget=Self::EventLoopTarget> + Deref<Target=Self::EventLoopTarget>;
    
    /// The renderer to use that supports the given surface
    type Renderer: Renderer<Self> + Send;

}