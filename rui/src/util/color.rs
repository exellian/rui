pub enum Color {
    RGBA {
        r: f32,
        g: f32,
        b: f32,
        a: f32
    },
    RGB {
        r: f32,
        g: f32,
        b: f32
    }
}
impl Color {

    pub const BLACK: Self = Color::RGB { r: 0.0, g: 0.0, b: 0.0 };

    pub fn as_raw(&self) -> [f32; 4] {
        match self {
            Color::RGBA { r, g, b, a } => {
                [*r, *g, *b, *a]
            }
            Color::RGB { r, g, b } => {
                [*r, *g, *b, 1.0]
            }
        }
    }
}
impl From<[u8;3]> for Color {
    fn from(c: [u8; 3]) -> Self {
        Color::RGB {
            r: c[0] as f32 / 255.0,
            g: c[1] as f32 / 255.0,
            b: c[2] as f32 / 255.0
        }
    }
}
impl From<[f32;3]> for Color {
    fn from(c: [f32; 3]) -> Self {
        Color::RGB {
            r: c[0],
            g: c[1],
            b: c[2]
        }
    }
}