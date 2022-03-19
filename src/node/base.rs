use crate::util::{Color, Extent, Flags, Offset, Rect};

pub struct BaseNode {
    flags: Flags,
    bounding_rect: Rect,
    background: Color,
}

impl Default for BaseNode {
    fn default() -> Self {
        BaseNode {
            flags: Flags::DEFAULT,
            bounding_rect: Rect {
                offset: Offset {
                    x: 0,
                    y: 0
                },
                extent: Extent {
                    width: 0,
                    height: 0
                }
            },
            background: ()
        }
    }
}