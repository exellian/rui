use windows_sys::Win32::Foundation::HWND;
use crate::os_error::OsError;

pub struct Surface {
    handle: HWND
}

impl<'a> TryFrom<SurfaceAttributes<'a>> for Surface {
    type Error = OsError;

    fn try_from(value: SurfaceAttributes<'a>) -> Result<Self, Self::Error> {
        todo!()
    }
}