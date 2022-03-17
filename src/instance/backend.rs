use crate::event::EventLoop;
use crate::renderer::Renderer;
use crate::surface::Surface;

pub trait Backend {
    /// Should be an enum type that
    /// allows the user to define own events
    type UserEvent: 'static;
    
    /// Surface type that should be used to create e.g: windows
    /// Default is the winit library
    type Surface: Surface<EventLoop=Self::EventLoop>;

    /// The event loop that should be used. Hint: it must be compatible 
    /// with the surface
    type EventLoop: EventLoop<Self::UserEvent, Surface=Self::Surface>;
    
    /// The renderer to use that supports the given surface
    type Renderer: Renderer<Self::Surface>;

}