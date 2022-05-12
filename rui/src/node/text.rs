use wgpu_glyph::ab_glyph::FontArc;

#[derive(Debug, Clone)]
pub struct TextNode {
    text: String,
    font_size: f32,
    font_resource: FontArc,
}

impl TextNode {
    pub fn new(text: String, font_size: f32, font_resource: FontArc) -> Self {
        TextNode {
            text,
            font_size,
            font_resource,
        }
    }

    pub fn text(&self) -> &str {
        &self.text
    }

    pub fn font_size(&self) -> f32 {
        self.font_size
    }

    pub fn font_resource(&self) -> FontArc {
        self.font_resource.clone()
    }
}
