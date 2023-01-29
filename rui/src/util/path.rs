use crate::util::point::Point2D;

#[derive(Clone)]
pub enum PathSegment {
    Linear { to: Point2D },
    Arc { to: Point2D, radii: [f32; 2] },
    QuadraticBezier { to: Point2D, param: Point2D },
    CubicBezier { to: Point2D, params: [Point2D; 2] },
    CatmullRom,
}
impl PathSegment {
    pub fn to(&self) -> &Point2D {
        match self {
            PathSegment::Linear { to } => to,
            PathSegment::Arc { to, .. } => to,
            PathSegment::QuadraticBezier { to, .. } => to,
            PathSegment::CubicBezier { to, .. } => to,
            PathSegment::CatmullRom => {
                unreachable!()
            }
        }
    }
}
