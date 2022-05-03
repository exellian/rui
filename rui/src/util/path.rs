use crate::util::point::Point2D;

pub enum PathSegment {
    Linear { to: Point2D },
    Arc { to: Point2D, radii: [f32; 2] },
    QuadraticBezier { to: Point2D, param: Point2D },
    CubicBezier { to: Point2D, params: [Point2D; 2] },
    CatmullRom,
}
