mod color;
mod flags;
mod handler;
mod id;
mod path;
mod point;
mod rect;
mod resource;
mod solver;

pub use color::Color;
pub use flags::Flags;
pub use handler::Handler;
pub use id::id;
pub use path::PathSegment;
pub use point::Point2D;
pub use rect::Rect;
pub use resource::Resource;

pub fn pack(x: u16, y: u16) -> u32 {
    ((x as u32) << 16) | (y as u32)
}
pub fn unpack(x: u32) -> (u16, u16) {
    ((x >> 16) as u16, (x & 0xFFFF) as u16)
}
