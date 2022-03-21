use std::sync::{Arc, Mutex};
use tokio::sync::RwLock;
use crate::instance::{Backend, Instance};
use crate::Node;
use crate::node::base::BaseNode;
use crate::renderer::Renderer;

pub struct Context<B> where B: Backend {
    base: BaseNode,
    instance: Instance<B>,
}

impl<B> Context<B> where B: Backend + 'static {

    async fn mount(&mut self, surface: &B::Surface, node: Node) {
        self.instance.mount(surface, node).await;
    }
}