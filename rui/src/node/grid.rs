use crate::node::Node;
#[allow(unused)]
pub struct GridNode {
    grid: Vec<Vec<Node>>,
}
#[allow(unused)]
impl GridNode {
    pub fn grid(&self) -> &Vec<Vec<Node>> {
        &self.grid
    }
}
