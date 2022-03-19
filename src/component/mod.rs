mod component;
pub mod context;

pub use component::Component;
use crate::Node;
use crate::node::base::BaseNode;

pub fn component<T>(component: T) -> Node where T: 'static + Component {
    Node::Component(BaseNode::default(), Box::new(component))
}