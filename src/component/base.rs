use crate::util::{Color, Flags, Rect};

pub struct BaseComponent {
    flags: Flags,
    bounding_rect: Rect,
    background: Color,
}