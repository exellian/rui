use crate::os_error::OsError;
use crate::surface::Surface;
use crate::surface::surface_attributes::SurfaceAttributes;

pub struct SurfaceBuilder<'a> {
    attributes: SurfaceAttributes<'a>
}
impl<'a> SurfaceBuilder<'a> {

    pub fn new() -> Self {
        SurfaceBuilder {
            attributes: SurfaceAttributes::default()
        }
    }
    
    pub fn build(self) -> Result<Surface, OsError> {
        Surface::try_from(&self.attributes)
    }
}