use crate::math::{Vec4};
use crate::Node;

pub struct BorderNode {
    width: Vec4,
    radii: Vec4,
    inset: bool,
    node: Box<Node>
}
impl BorderNode {

    pub fn width(&self) -> &Vec4 {
        &self.width
    }

    pub fn radii(&self) -> &Vec4 {
        &self.radii
    }

    pub fn inset(&self) -> bool {
        self.inset
    }
    
    pub fn node(&self) -> &Node {
        &self.node
    }

    pub fn node_mut(&mut self) -> &mut Node {
        &mut self.node
    }
}