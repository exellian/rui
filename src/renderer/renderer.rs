use std::error::Error;
use crate::node::Node;
use async_trait::async_trait;

#[async_trait]
pub trait Renderer<> {
    type Error: Error;

    async fn mount(&mut self, surface: &, node: &Node) -> Result<(), Self::Error>;
}