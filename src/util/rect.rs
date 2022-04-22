use std::cmp::{max, min};
use util::{Extent, Offset};

#[derive(Debug)]
pub struct Rect {
    pub offset: Offset,
    pub extent: Extent
}
impl Rect {

    pub fn new(x: i32, y: i32, width: u32, height: u32) -> Self {
        Rect {
            offset: Offset {
                x,
                y
            },
            extent: Extent {
                width,
                height
            }
        }
    }

    pub fn intersect(&self, other: &Self) -> Option<Self> {
        let x_upper = max(self.offset.x, other.offset.x);
        let y_upper = max(self.offset.y, other.offset.y);
        let x_lower = min(self.offset.x + self.extent.width as i32, other.offset.x + other.extent.width as i32);
        let y_lower = min(self.offset.y + self.extent.height as i32, other.offset.y + other.extent.height as i32);
        return if x_lower > x_upper && y_lower > y_upper {
            Some(Rect::new(x_upper, y_lower, (x_lower - x_upper) as u32, (y_lower - y_upper) as u32))
        } else {
            None
        }
    }

    pub fn norm(&self, norm: &Rect) -> [f32;4] {
        [
            self.offset.x as f32 / norm.extent.width as f32,
            self.offset.y as f32 / norm.extent.height as f32,
            self.extent.width as f32 / norm.extent.width as f32,
            self.extent.height as f32 / norm.extent.height as f32
        ]
    }

    pub fn as_raw(&self) -> [f32; 4] {
        [
            self.offset.x as f32,
            self.offset.y as f32,
            self.extent.width as f32,
            self.extent.height as f32
        ]
    }
}