use windows_sys::Win32::Foundation::{HWND};
use windows_sys::Win32::UI::WindowsAndMessaging::CreateWindowExW;
use crate::os_error::OsError;
use crate::surface::SurfaceAttributes;

pub struct Surface {
    handle: HWND
}

impl Surface {
    fn new(handle: HWND) -> Self {
        Surface {
            handle
        }
    }
}

impl<'a> TryFrom<&SurfaceAttributes<'a>> for Surface {
    type Error = OsError;

    fn try_from(value: &SurfaceAttributes<'a>) -> Result<Self, Self::Error> {

        let window_flags = WindowFlags::empty();

        let handle = unsafe {
            CreateWindowExW(

            )
        };
        todo!()
    }
}