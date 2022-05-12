use crate::util::{Color, Flags, Rect};
use rui_util::{Extent, Offset};

#[derive(Debug, Copy, Clone)]
pub struct BaseNode {
    pub(crate) flags: Flags,
    pub(crate) bounding_rect: Rect,
    pub(crate) background: Color,
    pub(crate) border_radii: [f32; 4],
}

impl Default for BaseNode {
    fn default() -> Self {
        BaseNode {
            flags: Flags::DEFAULT,
            bounding_rect: Rect {
                offset: Offset { x: 0, y: 0 },
                extent: Extent {
                    width: 0,
                    height: 0,
                },
            },
            background: Color::BLACK,
            border_radii: [0.0, 0.0, 0.0, 0.0],
        }
    }
}
