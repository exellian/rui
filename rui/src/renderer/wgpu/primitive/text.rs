use crate::node::text::TextNode;
use wgpu_glyph::ab_glyph::FontArc;

pub(crate) struct Text {
    content: String,
    font_size: f32,
    font: FontArc,
}

impl From<&mut TextNode> for Text {
    fn from(tn: &mut TextNode) -> Self {
        Text {
            content: tn.text().to_string(),
            font_size: tn.font_size(),
            font: tn.font_resource().clone(),
        }
    }
}

impl Text {
    pub fn content(&self) -> String {
        self.content.clone()
    }

    pub fn font_size(&self) -> f32 {
        self.font_size
    }

    pub fn font(&self) -> FontArc {
        self.font.clone()
    }
}
