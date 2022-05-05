use std::marker::PhantomData;
use std::pin::Pin;

use cocoa::appkit::{CGFloat, NSApp, NSBackingStoreBuffered, NSView, NSWindow, NSWindowStyleMask};
use cocoa::base::{id, nil};
use cocoa::foundation::{NSPoint, NSRect, NSSize};
use objc::rc::autoreleasepool;
use objc::runtime::{BOOL, NO};
use raw_window_handle::{AppKitHandle, HasRawWindowHandle, RawWindowHandle};

use class::Class as WindowClass;
use delegate_class::DelegateClass as WindowDelegateClass;
use delegate_state::DelegateState as WindowDelegateState;
use rui_util::Extent;
use view_class::ViewClass as WindowViewClass;

use crate::event::{InnerLoop, LoopTarget};
use crate::platform::event::Queue;
use crate::platform::platform::surface::delegate_state::DelegateState;
use crate::platform::platform::surface::view_state::ViewState;
use crate::platform::platform::util;
use crate::surface::{SurfaceAttributes, SurfaceId};

mod class;
mod delegate_class;
mod delegate_state;
mod view_class;
mod view_state;

pub struct Surface<'main, 'child> {
    ns_window: id,
    ns_view: id,
    ns_window_delegate: id,
    view_state: Pin<Box<ViewState>>,
    window_delegate_state: Pin<Box<DelegateState>>,
    queue: Queue,
    loop_target: LoopTarget<'main, 'child>,
    // Ensure that the window cannot be send between threads
    _non_send: PhantomData<*const ()>,
}
impl<'main, 'child> Surface<'main, 'child> {
    pub fn new(loop_target: &LoopTarget<'main, 'child>, attr: &SurfaceAttributes) -> Self {
        let is_main_thread: BOOL = unsafe { msg_send![class!(NSThread), isMainThread] };
        if is_main_thread == NO {
            panic!("Surface on macos can only be created with the main loop target!");
        }
        lazy_static! {
            static ref WINDOW_CLASS: WindowClass = WindowClass::new();
            static ref WINDOW_VIEW_CLASS: WindowViewClass = WindowViewClass::new();
            static ref WINDOW_DELEGATE_CLASS: WindowDelegateClass = WindowDelegateClass::new();
        }
        let main_loop = match loop_target {
            LoopTarget::Main(ml) => *ml,
            LoopTarget::Child(child) => child.main,
        };
        let (ns_window, ns_view, ns_window_delegate, view_state, window_delegate_state) =
            autoreleasepool(|| {
                let frame = NSRect::new(
                    NSPoint::new(attr.position.x as CGFloat, attr.position.y as CGFloat),
                    NSSize::new(
                        attr.current_size.width as CGFloat,
                        attr.current_size.height as CGFloat,
                    ),
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
                let ns_window_alloc: id = unsafe { msg_send![WINDOW_CLASS.0, alloc] };
                // TODO check if init is safe when ns_window == nil
                let ns_window = unsafe {
                    ns_window_alloc.initWithContentRect_styleMask_backing_defer_(
                        frame,
                        style,
                        NSBackingStoreBuffered,
                        NO,
                    )
                };

                if ns_window == nil {
                    panic!("Failed to allocate window!");
                }

                // NSView creation
                let mut view_state = Box::pin(ViewState::new(
                    ns_window,
                    main_loop.inner.borrow().queue.clone(),
                ));
                let view_state_ptr = unsafe { view_state.as_mut().get_unchecked_mut() as *mut _ };
                let ns_view_alloc: id = unsafe { msg_send![WINDOW_VIEW_CLASS.0, alloc] };
                let ns_view: id =
                    unsafe { msg_send![ns_view_alloc, initWithState: view_state_ptr] };
                unsafe { ns_window.setContentView_(ns_view) };
                unsafe { ns_window.setInitialFirstResponder_(ns_view) };

                // Window Delegate creation
                let mut window_delegate_state = Box::pin(WindowDelegateState::new(
                    ns_window,
                    ns_view,
                    main_loop.inner.borrow().queue.clone(),
                ));
                let window_delegate_state_ptr =
                    unsafe { window_delegate_state.as_mut().get_unchecked_mut() as *mut _ };
                let ns_window_delegate_alloc: id =
                    unsafe { msg_send![WINDOW_DELEGATE_CLASS.0, alloc] };
                let ns_window_delegate: id = unsafe {
                    msg_send![
                        ns_window_delegate_alloc,
                        initWithState: window_delegate_state_ptr
                    ]
                };
                unsafe { ns_window.setDelegate_(ns_window_delegate) };
                unsafe { ns_window.makeKeyAndOrderFront_(NSApp()) };
                (
                    ns_window,
                    ns_view,
                    ns_window_delegate,
                    view_state,
                    window_delegate_state,
                )
            });
        // window creation
        let window = Surface {
            ns_window,
            ns_view,
            ns_window_delegate,
            view_state,
            window_delegate_state,
            queue: main_loop.inner.borrow_mut().queue.clone(),
            loop_target: loop_target.clone(),
            _non_send: PhantomData,
        };
        window
    }

    pub fn id(&self) -> SurfaceId {
        util::get_window_id(self.ns_window)
    }

    pub fn scale_factor(&self) -> f64 {
        unsafe { NSWindow::backingScaleFactor(self.ns_window) as _ }
    }

    pub fn inner_size(&self) -> Extent {
        let view_frame = unsafe { NSView::frame(self.ns_view) };
        let scale_factor = self.scale_factor();
        let (x, y) = (view_frame.size.width as f64, view_frame.size.height as f64);
        Extent {
            width: (x * scale_factor) as u32,
            height: (y * scale_factor) as u32,
        }
    }

    pub fn request_redraw(&self) {
        match self.loop_target {
            LoopTarget::Main(main) => {
                let id = self.id();
                let mut mut_guard = main.inner.borrow_mut();
                if !mut_guard.redraw_pending.contains(&id) {
                    mut_guard.redraw_pending.push(id);
                }
                mut_guard.wake_up();
            }
            LoopTarget::Child(_) => unreachable!(),
        }
    }
}
unsafe impl<'main, 'child> HasRawWindowHandle for Surface<'main, 'child> {
    fn raw_window_handle(&self) -> RawWindowHandle {
        let mut handle = AppKitHandle::empty();
        handle.ns_window = self.ns_window as *mut _;
        handle.ns_view = self.ns_view as *mut _;
        RawWindowHandle::AppKit(handle)
    }
}
impl<'main, 'child> Drop for Surface<'main, 'child> {
    fn drop(&mut self) {
        // Because the window can only be created on the main thread and cannot be moved
        // to a different thread we can safely perform this close operation
        unsafe { self.ns_window.close() }
    }
}
