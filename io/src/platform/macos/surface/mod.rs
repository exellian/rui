use crate::os_error::OsError;
use crate::surface::SurfaceAttributes;

pub struct Surface {}


impl<'a> TryFrom<&SurfaceAttributes<'a>> for Surface {
    type Error = OsError;

    fn try_from(attr: &SurfaceAttributes<'a>) -> Result<Self, Self::Error> {

        Ok(Surface {})
    }
}