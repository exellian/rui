use crate::node::Node;

pub struct CompositionNode {
    layers: Vec<Node>
}
impl CompositionNode {
    
    pub fn layers(&self) -> &Vec<Node> {
        &self.layers
    }
}