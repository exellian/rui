mod window;

use crate::event::LoopTarget;
use crate::platform::platform::surface::window::{WindowClass, WindowInit};
use crate::platform::platform::util;
use crate::surface::{SurfaceAttributes, SurfaceId};
use lazy_static::lazy_static;
use raw_window_handle::{HasRawDisplayHandle, HasRawWindowHandle, RawDisplayHandle, RawWindowHandle, Win32WindowHandle, WindowsDisplayHandle};
use rui_util::Extent;
use std::{io, mem, ptr};
use windows_sys::Win32::Foundation::{HINSTANCE, HWND, RECT};
use windows_sys::Win32::Graphics::Gdi::{RedrawWindow, RDW_INTERNALPAINT};
use windows_sys::Win32::System::LibraryLoader::GetModuleHandleW;
use windows_sys::Win32::UI::WindowsAndMessaging::{
    CreateWindowExW, GetClientRect, RegisterClassExW, ShowWindow, CREATESTRUCTW, CS_HREDRAW,
    CS_VREDRAW, CW_USEDEFAULT, SW_SHOW, WNDCLASSEXW, WS_OVERLAPPEDWINDOW,
};

pub struct Surface<'main, 'child> {
    id: SurfaceId,
    handle: HWND,
    hinstance: HINSTANCE,
    loop_target: LoopTarget<'main, 'child>,
}

impl<'main, 'child> Surface<'main, 'child> {
    pub(crate) async fn new(
        loop_target: &LoopTarget<'main, 'child>,
        attr: &SurfaceAttributes,
    ) -> Surface<'main, 'child> {
        lazy_static! {
            static ref WINDOW_CLASS: WindowClass = unsafe { WindowClass::new() };
        }

        let title = util::encode_wide(attr.title.clone());

        let ex_style = 0;
        let mut style = 0;

        style |= WS_OVERLAPPEDWINDOW;

        // On windows multithreaded window creation is no problem
        // We only must ensure that we keep the thread affinity of the window.
        // But this is automatically ensured because each surface is not send and not sync
        // and therefore can't be moved or accessed by a different thread
        let callback = match loop_target {
            LoopTarget::Main(ml) => ml.inner.borrow_mut().inner.state_mut().callback.clone(),
            LoopTarget::Child(child) => child.inner.borrow_mut().inner.state_mut().callback.clone(),
        };

        let mut init_data = WindowInit::new(callback);

        let hinstance = unsafe { GetModuleHandleW(ptr::null()) };

        let handle = unsafe {
            match {
                CreateWindowExW(
                    ex_style,
                    WINDOW_CLASS.class_name().as_ptr(),
                    title.as_ptr(),
                    style,
                    CW_USEDEFAULT,
                    CW_USEDEFAULT,
                    CW_USEDEFAULT,
                    CW_USEDEFAULT,
                    0,
                    0,
                    hinstance,
                    &mut init_data as *mut _ as *mut _,
                )
            } {
                0 => Err(io::Error::last_os_error()),
                handle @ _ => Ok(handle),
            }
        }
        .unwrap();

        unsafe {
            ShowWindow(handle, SW_SHOW);
        }

        Surface {
            id: util::surface_id(handle),
            handle,
            hinstance,
            loop_target: loop_target.clone(),
        }
    }

    pub fn inner_size(&self) -> Extent {
        let mut rect: RECT = unsafe { mem::zeroed() };
        if unsafe { GetClientRect(self.handle, &mut rect) } == false.into() {
            panic!("Unexpected GetClientRect failure: please report this error to https://github.com/exellian/rui")
        }
        Extent {
            width: (rect.right - rect.left) as u32,
            height: (rect.bottom - rect.top) as u32,
        }
    }

    pub fn id(&self) -> SurfaceId {
        self.id
    }

    pub fn request_redraw(&mut self) {
        unsafe {
            RedrawWindow(self.handle, ptr::null(), 0, RDW_INTERNALPAINT);
        }
    }
}

unsafe impl<'main, 'child> HasRawWindowHandle for Surface<'main, 'child> {
    fn raw_window_handle(&self) -> RawWindowHandle {
        let mut handle = Win32WindowHandle::empty();
        handle.hwnd = self.handle as *mut _;
        handle.hinstance = self.hinstance as *mut _;
        RawWindowHandle::Win32(handle)
    }
}

unsafe impl<'main, 'child> HasRawDisplayHandle for Surface<'main, 'child> {
    fn raw_display_handle(&self) -> RawDisplayHandle {
        RawDisplayHandle::Windows(WindowsDisplayHandle::empty())
    }
}
