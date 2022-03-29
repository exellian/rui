pub struct ImageNode {
    resource: String,
}
impl ImageNode {

    pub fn resource(&self) -> &str {
        &self.resource
    }
}