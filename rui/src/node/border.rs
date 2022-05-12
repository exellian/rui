use crate::math::Vec4;
use crate::Node;

pub struct BorderNode {
    width: Vec4<f32>,
    radii: Vec4<f32>,
    inset: bool,
    node: Box<Node>,
}
impl BorderNode {
    pub fn width(&self) -> &Vec4<f32> {
        &self.width
    }

    pub fn radii(&self) -> &Vec4<f32> {
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
