use crate::util::Resource;

pub struct ImageNode {
    resource: Resource,
}
impl ImageNode {

    pub fn new(resource: Resource) -> Self {
        ImageNode {
            resource
        }
    }

    pub fn resource(&self) -> &Resource {
        &self.resource
    }
}