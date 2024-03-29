use rui_util::{Extent, Offset};
use std::cmp::{max, min};

#[derive(Debug)]
pub struct Rect {
    pub offset: Offset,
    pub extent: Extent,
}
impl Rect {
    pub fn new(x: i32, y: i32, width: u32, height: u32) -> Self {
        Rect {
            offset: Offset { x, y },
            extent: Extent { width, height },
        }
    }

    pub fn new_points(x_ul: u32, y_ul: u32, x_lr: u32, y_lr: u32) -> Self {
        Rect {
            offset: Offset {
                x: x_ul as i32,
                y: y_ul as i32,
            },
            extent: Extent {
                width: x_lr - x_ul,
                height: y_lr - y_ul,
            },
        }
    }

    pub fn intersect(&self, other: &Self) -> Option<Self> {
        let x_upper = max(self.offset.x, other.offset.x);
        let y_upper = max(self.offset.y, other.offset.y);
        let x_lower = min(
            self.offset.x + self.extent.width as i32,
            other.offset.x + other.extent.width as i32,
        );
        let y_lower = min(
            self.offset.y + self.extent.height as i32,
            other.offset.y + other.extent.height as i32,
        );
        return if x_lower > x_upper && y_lower > y_upper {
            Some(Rect::new(
                x_upper,
                y_lower,
                (x_lower - x_upper) as u32,
                (y_lower - y_upper) as u32,
            ))
        } else {
            None
        };
    }

    pub fn max(&self, other: &Self) -> Self {
        let offset_x_min = min(other.offset.x, self.offset.x);
        let offset_y_min = min(other.offset.y, self.offset.y);
        let extent_width_max = max(other.extent.width, self.extent.width);
        let extent_height_max = max(other.extent.height, self.extent.height);
        Rect::new(
            offset_x_min,
            offset_y_min,
            extent_width_max,
            extent_height_max,
        )
    }
    pub fn min(&self, other: &Self) -> Self {
        let offset_x_max = max(other.offset.x, self.offset.x);
        let offset_y_max = max(other.offset.y, self.offset.y);
        let extent_width_min = min(other.extent.width, self.extent.width);
        let extent_height_min = min(other.extent.height, self.extent.height);
        Rect::new(
            offset_x_max,
            offset_y_max,
            extent_width_min,
            extent_height_min,
        )
    }

    pub fn norm(&self, norm: &Rect) -> [f32; 4] {
        [
            self.offset.x as f32 / norm.extent.width as f32,
            self.offset.y as f32 / norm.extent.height as f32,
            self.extent.width as f32 / norm.extent.width as f32,
            self.extent.height as f32 / norm.extent.height as f32,
        ]
    }

    pub fn as_raw(&self) -> [f32; 4] {
        [
            self.offset.x as f32,
            self.offset.y as f32,
            self.offset.x as f32 + self.extent.width as f32,
            self.offset.y as f32 + self.extent.height as f32,
        ]
    }
}
impl Default for Rect {
    fn default() -> Self {
        Rect::new(0, 0, 0, 0)
    }
}
