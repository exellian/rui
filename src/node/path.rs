use crate::util::PathSegment;

pub struct PathNode {
    segments: Vec<PathSegment>
}
impl PathNode {
    
    pub fn new(segments: Vec<PathSegment>) -> Self {
        PathNode {
            segments
        }
    }
    
    pub fn segments(&self) -> &Vec<PathSegment> {
        &self.segments
    }
}