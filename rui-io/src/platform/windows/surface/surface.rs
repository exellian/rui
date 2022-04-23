use std::{io, mem, ptr};
use windows_sys::Win32::Foundation::{HWND};
use windows_sys::Win32::System::LibraryLoader::GetModuleHandleW;
use windows_sys::Win32::UI::WindowsAndMessaging::{CreateWindowExW, CS_HREDRAW, CS_VREDRAW, CW_USEDEFAULT, RegisterClassExW, WNDCLASSEXW};
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

    unsafe fn init(attr: &SurfaceAttributes<'_>) -> Result<HWND, io::Error> {
        let class_name = Self::register_window_class("");
        let title = util::encode_wide(attr.title.as_ref());
        let ex_style = 0;
        let style = 0;

        match unsafe {
            CreateWindowExW(
                ex_style,
                class_name.as_ptr(),
                title.as_ptr(),
                style,
                CW_USEDEFAULT,
                CW_USEDEFAULT,
                CW_USEDEFAULT,
                CW_USEDEFAULT,
                0,
                0,
                GetModuleHandleW(ptr::null()),
                ptr::null()
            )
        } {
            0 => Err(io::Error::last_os_error()),
            handle@_ => Ok(handle)
        }
    }

    unsafe fn register_window_class(class_name: &str) -> Vec<u16> {
        let class_name = util::encode_wide(class_name);

        let class = WNDCLASSEXW {
            cbSize: mem::size_of::<WNDCLASSEXW>() as u32,
            style: CS_HREDRAW | CS_VREDRAW,
            lpfnWndProc: None,
            cbClsExtra: 0,
            cbWndExtra: 0,
            hInstance: GetModuleHandleW(ptr::null()),
            hIcon: 0,
            hCursor: 0, // must be null in order for cursor state to work properly
            hbrBackground: 0,
            lpszMenuName: ptr::null(),
            lpszClassName: class_name.as_ptr(),
            hIconSm: 0,
        };

        // We ignore errors because registering the same window class twice would trigger
        //  an error, and because errors here are detected during CreateWindowEx anyway.
        // Also since there is no weird element in the struct, there is no reason for this
        //  call to fail.
        RegisterClassExW(&class);

        class_name
    }
}

impl<'a> TryFrom<&SurfaceAttributes<'a>> for Surface {
    type Error = OsError;

    fn try_from(attr: &SurfaceAttributes<'a>) -> Result<Self, Self::Error> {

        let handle = unsafe { Surface::init(attr) }?;
        Ok(Surface::new(handle))
    }
}