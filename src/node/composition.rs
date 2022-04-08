use crate::node::Node;

pub struct CompositionNode {
    layers: Vec<Node>
}
impl CompositionNode {

    pub fn new(layers: Vec<Node>) -> Self {
        CompositionNode {
            layers
        }
    }

    pub fn layers(&self) -> &Vec<Node> {
        &self.layers
    }

    pub fn layers_mut(&mut self) -> &mut Vec<Node> {
        &mut self.layers
    }
}