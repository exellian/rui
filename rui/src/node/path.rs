use crate::Node;
use crate::node::base::BaseNode;
use crate::util::{PathSegment, Point2D};

pub struct Builder {
    base: BaseNode,
    from: Point2D,
    segments: Vec<PathSegment>
}
impl Builder {
    
    pub fn new(base: BaseNode, from: Point2D) -> Self {
        Builder {
            base,
            from,
            segments: vec![]
        }
    }
    
    pub fn cubic_bezier(mut self, to: impl Into<Point2D>, a: impl Into<Point2D>, b: impl Into<Point2D>) -> Self {
        self.segments.push(PathSegment::CubicBezier {
            to: to.into(),
            params: [
                a.into(),
                b.into()
            ]
        });
        self
    }
    
    pub fn linear(mut self, to: impl Into<Point2D>) -> Self {
        self.segments.push(PathSegment::Linear {
            to: to.into()
        });
        self
    }
    
    pub fn build(self) -> Node {
        Node::Path(self.base, PathNode {
            from: self.from,
            segments: self.segments
        })
    }
    
    pub fn close(mut self) -> Node {
        self.segments.push(PathSegment::Linear {
            to: self.from
        });
        Node::Path(self.base, PathNode {
            from: self.from,
            segments: self.segments
        })
    }
}

pub struct PathNode {
    from: Point2D,
    segments: Vec<PathSegment>
}
impl PathNode {
    
    pub fn new(from: Point2D, segments: Vec<PathSegment>) -> Self {
        PathNode {
            from,
            segments
        }
    }
    
    pub fn builder(base: BaseNode, from: impl Into<Point2D>) -> Builder {
        Builder::new(base, from.into())
    }

    pub fn from(&self) -> &Point2D {
        &self.from
    }

    pub fn segments(&self) -> &Vec<PathSegment> {
        &self.segments
    }
}