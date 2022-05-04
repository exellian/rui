use crate::node::Node;
use crate::surface::Surface;
use crate::Backend;
use async_trait::async_trait;
use rui_io::surface::SurfaceId;
use rui_util::Extent;
use std::error::Error;
use std::fmt::Debug;
use std::future::Future;
use std::pin::Pin;
use std::sync::Arc;

pub trait Renderer<B>
where
    B: Backend,
{
    type Error: Error + Debug;

    fn mount(
        &mut self,
        surface: &rui_io::surface::Surface,
        node: &mut Node,
    ) -> Result<(), Self::Error>;
    fn resize(
        &mut self,
        surface: &rui_io::surface::Surface,
        size: Extent,
    ) -> Result<(), Self::Error>;
    fn render(&mut self, surface: &rui_io::surface::Surface) -> Result<(), Self::Error>;
    fn request_render(&self) -> Result<(), Self::Error>;
}
