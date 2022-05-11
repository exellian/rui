use crate::event::Event;
use crate::platform::platform::surface::window::{WindowData, WindowInit};
use crate::platform::platform::util;
use crate::surface::SurfaceEvent;
use rui_util::Extent;
use std::cell::Cell;
use std::sync::atomic::{AtomicUsize, Ordering};
use std::{mem, ptr};
use windows_sys::Win32::Foundation::{HWND, LPARAM, LRESULT, WPARAM};
use windows_sys::Win32::System::LibraryLoader::GetModuleHandleW;
use windows_sys::Win32::UI::WindowsAndMessaging::{
    DefWindowProcW, RegisterClassExW, CREATESTRUCTW, CS_HREDRAW, CS_VREDRAW, GWL_USERDATA,
    WM_CREATE, WM_NCCREATE, WM_NCDESTROY, WM_PAINT, WM_SIZE, WNDCLASSEXW,
};

pub struct Class {
    class_name: Vec<u16>,
}
impl Class {
    pub(super) unsafe extern "system" fn window_callback(
        handle: HWND,
        msg: u32,
        wparam: WPARAM,
        lparam: LPARAM,
    ) -> LRESULT {
        let mut userdata_ptr = util::get_window_long(handle, GWL_USERDATA);
        let userdata = match (userdata_ptr, msg) {
            (0, msg) => {
                match msg {
                    WM_NCCREATE => {
                        let createstruct = &mut *(lparam as *mut CREATESTRUCTW);
                        let init_data = &mut *(createstruct.lpCreateParams as *mut WindowInit);
                        // Create raw heap allocated pointer for WindowData
                        // This gets later dropped manually in the WM_NCDESTROY event
                        let userdata_ptr =
                            Box::into_raw(Box::new(WindowData::new(init_data.callback.clone())))
                                as *mut WindowData as isize;
                        util::set_window_long(handle, GWL_USERDATA, userdata_ptr);
                    }
                    WM_CREATE => return -1,
                    _ => {}
                }
                return DefWindowProcW(handle, msg, wparam, lparam);
            }
            (_, WM_CREATE) => {
                // TODO
                return DefWindowProcW(handle, msg, wparam, lparam);
            }
            _ => &mut *(userdata_ptr as *mut WindowData),
        };

        match msg {
            WM_NCDESTROY => {
                {
                    // Reset the user data
                    util::set_window_long(handle, GWL_USERDATA, 0);
                    Box::from_raw(userdata);
                    // Userdata gets dropped here
                }
            }
            WM_PAINT => userdata.call(&Event::SurfaceEvent {
                id: util::surface_id(handle),
                event: SurfaceEvent::Redraw,
            }),
            WM_SIZE => {
                let width = util::loword(lparam as u32) as u32;
                let height = util::hiword(lparam as u32) as u32;

                userdata.call(&Event::SurfaceEvent {
                    id: util::surface_id(handle),
                    event: SurfaceEvent::Resized(Extent { width, height }),
                })
            }
            _ => {}
        }

        DefWindowProcW(handle, msg, wparam, lparam)
    }

    pub unsafe fn new() -> Self {
        static IDS: AtomicUsize = AtomicUsize::new(0);
        let id = IDS.fetch_add(1, Ordering::Acquire);

        let class_name = util::encode_wide(format!("WindowClass{}", id));

        let class = WNDCLASSEXW {
            cbSize: mem::size_of::<WNDCLASSEXW>() as u32,
            style: CS_HREDRAW | CS_VREDRAW,
            lpfnWndProc: Some(Self::window_callback),
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

        Class { class_name }
    }

    pub fn class_name(&self) -> &Vec<u16> {
        &self.class_name
    }
}
