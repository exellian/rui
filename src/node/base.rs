use crate::util::{Color, Flags, Rect};

pub struct BaseNode {
    flags: Flags,
    bounding_rect: Rect,
    background: Color,
}