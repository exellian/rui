use raw_window_handle::{HasRawWindowHandle, RawWindowHandle};
use crate::os_error::OsError;
use crate::surface::SurfaceAttributes;

pub struct Surface {}

unsafe impl HasRawWindowHandle for Surface {
    fn raw_window_handle(&self) -> RawWindowHandle {
        todo!()
    }
}

impl<'a> TryFrom<&SurfaceAttributes<'a>> for Surface {
    type Error = OsError;

    fn try_from(attr: &SurfaceAttributes<'a>) -> Result<Self, Self::Error> {

        Ok(Surface {})
    }
}