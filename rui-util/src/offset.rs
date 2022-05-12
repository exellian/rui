#[derive(Clone, Copy, Debug)]
pub struct Offset {
    pub x: i32,
    pub y: i32,
}

impl Into<(f32, f32)> for &Offset {
    fn into(self) -> (f32, f32) {
        (self.x as f32, self.y as f32)
    }
}
