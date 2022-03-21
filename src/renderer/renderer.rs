use std::error::Error;
use std::fmt::Debug;
use crate::node::Node;
use async_trait::async_trait;
use crate::Backend;

#[async_trait]
pub trait Renderer<B> where B: Backend {
    type Error: Error + Debug;

    async fn mount(&mut self, surface: &B::Surface, node: &Node) -> Result<(), Self::Error>;
}