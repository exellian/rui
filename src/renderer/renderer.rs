use std::error::Error;
use crate::node::Node;
use crate::surface::Surface;
use async_trait::async_trait;

#[async_trait]
pub trait Renderer<T> where T: Surface {
    type Error: Error;
    type SurfaceHandle: Copy;
    
    async fn mount_surface(&mut self, surface: &T) -> Result<Self::SurfaceHandle, Self::Error>;
    async fn mount_component(&mut self, surface: Self::SurfaceHandle, component: &Node) -> Result<(), Self::Error>;
}