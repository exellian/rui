use std::error::Error;
use crate::event::EventLoop;
use crate::surface::builder::Builder;
use crate::surface::surface_adapter::SurfaceAdapter;
use crate::surface::surface_attributes::SurfaceAttributes;

/// The surface factory is an abstraction for surface creation
/// that allows us to easily replace this process by any library
/// Search factory-pattern for more info.
///
/// # Generics
/// * `T` - A user defined type to enable own events
pub trait SurfaceFactory<T> where T: 'static {

    /// The type for the event loop that runs on the main thread.
    /// It should handle Input, Os, Window Events, etc...
    /// It should be compatible with the surface type that is
    /// instantiated by the factory
    type EventLoop: EventLoop<T, Surface=Self::Surface>;

    /// Error type specific to the surface
    type Error: Error;

    /// The surface type that is created by the factory
    /// e.g: winit::window::Window
    type Surface: SurfaceAdapter;

    /// Returns the created surface or an error if the creation failed
    ///
    /// # Arguments
    /// * `attributes` - The surface builder that contains all of the surface params
    /// * `event_loop` - The event loop that should handle the events from the surface
    fn build(event_loop: &Self::EventLoop, attributes: SurfaceAttributes) -> Result<Self::Surface, Self::Error>;
}