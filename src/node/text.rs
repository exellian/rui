pub struct TextNode {
    text: String,
    font_size: usize,
    font_resource: String
}
impl TextNode {

    pub fn text(&self) -> &str {
        &self.text
    }

    pub fn font_size(&self) -> usize {
        self.font_size
    }

    pub fn font_resource(&self) -> &str {
        &self.font_resource
    }
}