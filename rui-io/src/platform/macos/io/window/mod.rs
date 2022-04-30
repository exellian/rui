mod delegate_class;
mod delegate_state;
mod class;
mod view_class;
mod view_state;

use std::os::raw::c_void;
use std::sync::atomic::{AtomicU64, Ordering};
use cocoa::appkit::{CGFloat, NSBackingStoreBuffered, NSWindow, NSWindowStyleMask};
use cocoa::base::{id, nil};
use cocoa::foundation::{NSPoint, NSRect, NSSize};
use objc::rc::autoreleasepool;
use objc::runtime::{BOOL, NO};
use crate::surface::{SurfaceAttributes, SurfaceId};
use class::Class as WindowClass;
use delegate_state::DelegateState as WindowDelegateState;
use delegate_class::DelegateClass as WindowDelegateClass;
use view_class::ViewClass as WindowViewClass;
use crate::event::LoopTarget;
use crate::platform::io::window::delegate_state::DelegateState;
use crate::platform::io::window::view_state::ViewState;

pub struct Window<'main, 'child> {
    id: SurfaceId,
    ns_window: id,
    ns_view: id,
    view_state: Box<ViewState<'main, 'child>>,
    delegate_state: Box<DelegateState<'main, 'child>>
}
impl<'main, 'child> Window<'main, 'child> {

    pub fn new(attr: SurfaceAttributes, loop_target: &LoopTarget<'main, 'child>) -> Self {
        let is_main_thread: BOOL = unsafe { msg_send![class!(NSThread), isMainThread] };
        if is_main_thread == NO {
            panic!("Event Loop must be started from the main thread!");
        }
        lazy_static! {
            static ref WINDOW_CLASS: WindowClass = WindowClass::new();
            static ref WINDOW_VIEW_CLASS: WindowViewClass = WindowViewClass::new();
            static ref WINDOW_DELEGATE_CLASS: WindowDelegateClass = WindowDelegateClass::new();
        }
        static IDS: AtomicU64 = AtomicU64::new(0);
        let id = IDS.fetch_add(1, Ordering::Acquire);
        let ns_window = autoreleasepool(|| {
            let frame = NSRect::new(
                NSPoint::new(attr.position.x as CGFloat, attr.position.y as CGFloat),
                NSSize::new(attr.current_size.width as CGFloat, attr.current_size.height as CGFloat)
            );
            let mut style = NSWindowStyleMask::empty();
            if !attr.title.is_empty() {
                style |= NSWindowStyleMask::NSTitledWindowMask;
            }
            if attr.is_borderless {
                style |= NSWindowStyleMask::NSBorderlessWindowMask;
            }
            if attr.is_resizable {
                style |= NSWindowStyleMask::NSResizableWindowMask;
            }
            if attr.has_close_button {
                style |= NSWindowStyleMask::NSClosableWindowMask;
            }
            let ns_window: id = unsafe { msg_send![WINDOW_CLASS.as_objc_class(), alloc] };
            // TODO check if init is safe when ns_window == nil
            unsafe {
                ns_window.initWithContentRect_styleMask_backing_defer_(
                    frame,
                    style,
                    NSBackingStoreBuffered,
                    NO
                );
            }
            if ns_window != nil {
                panic!("Failed to allocate window!");
            }
            ns_window
        });
        // NSView creation
        let view_state = Box::new(ViewState::new(ns_window, loop_target.clone()));
        let view_ptr = &*view_state as *const ViewState as *const c_void;
        let ns_view_alloc: id = unsafe { msg_send![WINDOW_CLASS.as_objc_class(), alloc] };
        let ns_view: id = unsafe { msg_send![ns_view_alloc, initWithState: view_ptr] };

        // Window Delegate creation
        let delegate_state = Box::new(WindowDelegateState::new(ns_window, ns_view, loop_target.clone()));
        let delegate_state_ptr = &*delegate_state as *const DelegateState as *const c_void;
        let window_delegate_alloc: id = unsafe { msg_send![WINDOW_DELEGATE_CLASS.as_objc_class(), alloc] };
        let window_delegate: id = unsafe { msg_send![window_delegate_alloc, initWithState: delegate_state_ptr] };
        unsafe { ns_window.setDelegate_(window_delegate) };

        // window creation
        let window = Window {
            id: SurfaceId::from(id),
            ns_window,
            ns_view,
            view_state,
            delegate_state
        };

        window
    }

}