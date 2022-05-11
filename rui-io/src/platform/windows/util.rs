use crate::surface::SurfaceId;
use std::ffi::OsStr;
use std::iter::once;
use std::os::windows::ffi::OsStrExt;
use windows_sys::Win32::Foundation::HWND;
use windows_sys::Win32::UI::WindowsAndMessaging::WINDOW_LONG_PTR_INDEX;

pub fn encode_wide(str: impl AsRef<OsStr>) -> Vec<u16> {
    str.as_ref().encode_wide().chain(once(0)).collect()
}

pub fn surface_id(hwnd: HWND) -> SurfaceId {
    SurfaceId::from(hwnd as u64)
}

#[inline(always)]
pub const fn loword(x: u32) -> u16 {
    (x & 0xFFFF) as u16
}

#[inline(always)]
pub const fn hiword(x: u32) -> u16 {
    ((x >> 16) & 0xFFFF) as u16
}

#[inline(always)]
pub unsafe fn set_window_long(
    hwnd: HWND,
    nindex: WINDOW_LONG_PTR_INDEX,
    dwnewlong: isize,
) -> isize {
    #[cfg(target_pointer_width = "64")]
    return windows_sys::Win32::UI::WindowsAndMessaging::SetWindowLongPtrW(hwnd, nindex, dwnewlong);
    #[cfg(target_pointer_width = "32")]
    return windows_sys::Win32::UI::WindowsAndMessaging::SetWindowLongW(
        hwnd,
        nindex,
        dwnewlong as i32,
    ) as isize;
}

#[inline(always)]
pub unsafe fn get_window_long(hwnd: HWND, nindex: WINDOW_LONG_PTR_INDEX) -> isize {
    #[cfg(target_pointer_width = "64")]
    return windows_sys::Win32::UI::WindowsAndMessaging::GetWindowLongPtrW(hwnd, nindex);
    #[cfg(target_pointer_width = "32")]
    return windows_sys::Win32::UI::WindowsAndMessaging::GetWindowLongW(hwnd, nindex) as isize;
}
