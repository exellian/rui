use crate::os_error::OsError;
use crate::platform;
use crate::surface::surface_attributes::SurfaceAttributes;
use crate::surface::surface_builder::SurfaceBuilder;

pub struct Surface(platform::Surface);

impl Surface {

    fn new(surface: platform::Surface) -> Self {
        Surface(surface)
    }

    pub fn builder() -> SurfaceBuilder {
        SurfaceBuilder::new()
    }
}

impl<'a> TryFrom<SurfaceAttributes<'a>> for Surface {
    type Error = OsError;

    fn try_from(value: SurfaceAttributes<'a>) -> Result<Self, Self::Error> {
        let surface = platform::Surface::try_from(value)?;
        Ok(Surface::new(surface))
    }
}