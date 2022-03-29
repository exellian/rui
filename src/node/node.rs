use crate::component::Component;
use crate::node::base::BaseNode;
use crate::node::border::BorderNode;
use crate::node::composition::CompositionNode;
use crate::node::image::ImageNode;
use crate::node::text::TextNode;

pub enum Node {
    Rectangle(BaseNode),
    Border(BaseNode, BorderNode),
    Composition(BaseNode, CompositionNode),
    Image(BaseNode, ImageNode),
    Text(BaseNode, TextNode),
    Component(BaseNode, Box<dyn Component + Sync + Send>)
}
