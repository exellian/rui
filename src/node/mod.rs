mod node;
mod composition;
mod border;
pub(crate) mod base;
mod grid;
pub mod image;
pub mod text;

pub use node::Node;
use crate::Component;

use crate::node::base::BaseNode;
use crate::util::Color;

pub fn component<T>(component: T) -> Node where T: 'static + Component {
    Node::Component(BaseNode::default(), Box::new(component))
}

pub fn rect<C>(color: C, radii: [f32;4]) -> Node where C: Into<Color> {
    let mut base = BaseNode::default();
    base.background = color.into();
    base.border_radii = radii;
    Node::Rectangle(base)
}