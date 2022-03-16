pub enum Color {
    RGBNorm32 {
        r: f32,
        g: f32,
        b: f32
    },
    RGB {
        r: u8,
        g: u8,
        b: u8
    }
}
impl From<[f32;3]> for Color {
    fn from(c: [f32; 3]) -> Self {
        Color::RGBNorm32 {
            r: c[0],
            g: c[1],
            b: c[2]
        }
    }
}