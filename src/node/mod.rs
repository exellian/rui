mod node;
mod composition;
mod border;
pub(crate) mod base;
mod grid;
pub mod image;
pub mod text;

use std::path::{Path, PathBuf};
pub use node::Node;
use crate::Component;

use crate::node::base::BaseNode;
use crate::node::image::ImageNode;
use crate::util::{Color, Resource};

pub fn component<T>(component: T) -> Node where T: 'static + Component {
    Node::Component(BaseNode::default(), Box::new(component))
}

pub fn rect<C>(color: C, radii: [f32;4]) -> Node where C: Into<Color> {
    let mut base = BaseNode::default();
    base.background = color.into();
    base.border_radii = radii;
    Node::Rectangle(base)
}

pub fn image(image: impl AsRef<Path>, radii: [f32;4]) -> Node {
    let mut base = BaseNode::default();
    base.border_radii = radii;
    Node::Image(base, ImageNode::new(Resource::Path(image.as_ref().to_path_buf())))
}