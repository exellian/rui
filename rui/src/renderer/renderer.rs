use crate::node::Node;
use crate::Backend;
use rui_util::Extent;
use std::error::Error;
use std::fmt::Debug;

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
