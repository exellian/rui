use crate::util::point::Point2D;

pub enum PathSegment {
    Linear,
    Arc {
        start: Point2D,
        end: Point2D,
        radii: [f32;2]
    },
    QuadraticBezier {
        start: Point2D,
        end: Point2D,
        param: Point2D
    },
    CubicBezier {
        start: Point2D,
        end: Point2D,
        params: [Point2D;2]
    },
    CatmullRom
}