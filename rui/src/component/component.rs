use crate::instance::backend::Backend;
use crate::node::Node;
use async_trait::async_trait;

/// A component represents a collection of nodes
/// and should hold the state of the component
/// and should handle incoming events on the component
#[async_trait]
pub trait Component: Sync + Send {
    async fn on_event<B>(&mut self)
    where
        Self: Sized,
        B: Backend;

    async fn node(&mut self) -> Node;
}
