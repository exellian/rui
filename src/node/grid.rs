use crate::node::Node;

pub struct GridNode {
    grid: Vec<Vec<Node>>
}
impl GridNode {
    
    pub fn grid(&self) -> &Vec<Vec<Node>> {
        &self.grid
    }
}