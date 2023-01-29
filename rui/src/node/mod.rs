pub(crate) mod base;
mod border;
mod composition;
mod grid;
pub mod image;
mod node;
pub mod path;
pub mod text;

use crate::Component;
pub use node::Node;
use std::path::Path;

use crate::node::base::BaseNode;
use crate::node::composition::CompositionNode;
use crate::node::image::ImageNode;
use crate::node::path::PathNode;
use crate::util::{Color, Point2D, Resource};

pub fn component<T>(component: T) -> Node
where
    T: 'static + Component,
{
    Node::Component(BaseNode::default(), Box::new(component))
}

pub fn rect(color: impl Into<Color>, radii: [f32; 4]) -> Node {
    let mut base = BaseNode::default();
    base.background = color.into();
    base.border_radii = radii;
    Node::Rectangle(base)
}

pub fn image(image: impl AsRef<Path>, radii: [f32; 4]) -> Node {
    let mut base = BaseNode::default();
    base.border_radii = radii;
    Node::Image(
        base,
        ImageNode::new(Resource::Path(image.as_ref().to_path_buf())),
    )
}

pub fn comp(layers: impl Into<Vec<Node>>) -> Node {
    let base = BaseNode::default();
    Node::Composition(base, CompositionNode::new(layers.into()))
}

pub fn path(color: impl Into<Color>, from: impl Into<Point2D>) -> path::Builder {
    let mut base = BaseNode::default();
    base.background = color.into();
    PathNode::builder(base, from)
}
