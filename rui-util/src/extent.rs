#[derive(Debug, Clone, Copy)]
pub struct Extent {
    pub width: u32,
    pub height: u32,
}

impl Into<(f32, f32)> for &Extent {
    fn into(self) -> (f32, f32) {
        (self.width as f32, self.height as f32)
    }
}
