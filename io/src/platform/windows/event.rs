use windows_sys::Win32::UI::{
    Controls::WM_MOUSELEAVE,
    WindowsAndMessaging::{
        WM_ENTERSIZEMOVE,
        WM_EXITSIZEMOVE,
        WM_NCLBUTTONDOWN,
        WM_CLOSE,
        WM_DESTROY,
        WM_NCDESTROY,
        WM_PAINT,
        WM_WINDOWPOSCHANGING,
        WM_WINDOWPOSCHANGED,
        WM_SIZE,
        WM_CHAR,
        WM_SYSCHAR,
        WM_SYSCOMMAND,
        WM_MOUSEMOVE,
        WM_MOUSEWHEEL,
        WM_MOUSEHWHEEL,
        WM_KEYDOWN,
        WM_SYSKEYDOWN,
        WM_KEYUP,
        WM_SYSKEYUP,
        WM_LBUTTONDOWN,
        WM_LBUTTONUP,
        WM_RBUTTONDOWN,
        WM_RBUTTONUP,
        WM_MBUTTONDOWN,
        WM_MBUTTONUP,
        WM_XBUTTONDOWN,
        WM_XBUTTONUP,
        WM_CAPTURECHANGED,
        WM_TOUCH,
        WM_POINTERDOWN,
        WM_POINTERUPDATE,
        WM_POINTERUP,
        WM_SETFOCUS,
        WM_KILLFOCUS,
        WM_SETCURSOR,
        WM_DROPFILES,
        WM_GETMINMAXINFO,
        WM_DPICHANGED,
        WM_SETTINGCHANGE
    }
};
use crate::event::Event;

impl From<u32> for Event {

    fn from(msg: u32) -> Self {
        match msg {
            WM_ENTERSIZEMOVE => { Event::Default }

            WM_EXITSIZEMOVE => { Event::Default }

            WM_NCLBUTTONDOWN => { Event::Default }

            WM_CLOSE => { Event::Default }

            WM_DESTROY => { Event::Default }

            WM_NCDESTROY => { Event::Default }

            WM_PAINT => { Event::Default }

            WM_WINDOWPOSCHANGING => { Event::Default },

            // WM_MOVE supplies client area positions, so we send Moved here instead.
            WM_WINDOWPOSCHANGED => { Event::Default },

            WM_SIZE => { Event::Default },

            WM_CHAR | WM_SYSCHAR => { Event::Default },

            // this is necessary for us to maintain minimize/restore state
            WM_SYSCOMMAND => { Event::Default },

            WM_MOUSEMOVE => { Event::Default },

            WM_MOUSELEAVE => { Event::Default },

            WM_MOUSEWHEEL => { Event::Default },

            WM_MOUSEHWHEEL => { Event::Default },

            WM_KEYDOWN | WM_SYSKEYDOWN => { Event::Default },

            WM_KEYUP | WM_SYSKEYUP => { Event::Default },

            WM_LBUTTONDOWN => { Event::Default },

            WM_LBUTTONUP => { Event::Default },

            WM_RBUTTONDOWN => { Event::Default },

            WM_RBUTTONUP => { Event::Default },

            WM_MBUTTONDOWN => { Event::Default },

            WM_MBUTTONUP => { Event::Default },

            WM_XBUTTONDOWN => { Event::Default },

            WM_XBUTTONUP => { Event::Default },

            WM_CAPTURECHANGED => { Event::Default },

            WM_TOUCH => { Event::Default },

            WM_POINTERDOWN | WM_POINTERUPDATE | WM_POINTERUP => { Event::Default },

            WM_SETFOCUS => { Event::Default },

            WM_KILLFOCUS => { Event::Default },

            WM_SETCURSOR => { Event::Default },

            WM_DROPFILES => { Event::Default },

            WM_GETMINMAXINFO => { Event::Default },

            // Only sent on Windows 8.1 or newer. On Windows 7 and older user has to log out to change
            // DPI, therefore all applications are closed while DPI is changing.
            WM_DPICHANGED => { Event::Default }

            WM_SETTINGCHANGE => { Event::Default }
            _ => {
                Event::Default
            }
        }
    }
}