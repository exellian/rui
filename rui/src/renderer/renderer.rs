use std::error::Error;
use std::fmt::Debug;
use std::sync::Arc;
use crate::node::Node;
use async_trait::async_trait;
use rui_io::surface::SurfaceId;
use rui_util::Extent;
use crate::Backend;
use crate::surface::Surface;

#[async_trait]
pub trait Renderer<B> where B: Backend {
    type Error: Error + Debug;

    async fn mount(&mut self, surface: Arc<Surface>, node: &mut Node) -> Result<(), Self::Error>;
    async fn resize(&mut self, surface_id: SurfaceId, size: Extent) -> Result<(), Self::Error>;
    fn render(&self, surface_id: SurfaceId) -> Result<(), Self::Error>;
    fn request_render(&self) -> Result<(), Self::Error>;
}