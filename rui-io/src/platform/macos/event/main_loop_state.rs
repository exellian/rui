use crate::event::Event;
use crate::platform::event::app::{AppClass, AppDelegateClass, AppDelegateState};
use crate::surface::SurfaceId;
use cocoa::appkit::NSApp;
use cocoa::base::id;
use objc::rc::autoreleasepool;
use objc::runtime::{BOOL, NO};
use std::cell::RefCell;
use std::mem;
use std::pin::Pin;
use std::rc::Rc;

pub struct MainLoopState {
    pub(crate) ns_app: id,
    pub(crate) ns_app_delegate: id,
    pub(crate) app_delegate_state: Pin<Box<AppDelegateState>>,
    pub(crate) callback: Rc<RefCell<dyn FnMut(&Event)>>,
    pub(crate) redraw_pending: Vec<SurfaceId>,
}
impl MainLoopState {
    pub fn new(callback: impl FnMut(&Event)) -> Self {
        let is_main_thread: BOOL = unsafe { msg_send!(class!(NSThread), isMainThread) };
        if is_main_thread == NO {
            panic!("On macOS, `EventLoop` must be created on the main thread!");
        }

        lazy_static! {
            static ref APP_CLASS: AppClass = AppClass::new();
            static ref APP_DELEGATE_CLASS: AppDelegateClass = AppDelegateClass::new();
        }

        let callback = unsafe {
            mem::transmute::<Rc<RefCell<dyn FnMut(&Event)>>, Rc<RefCell<dyn FnMut(&Event)>>>(
                Rc::new(RefCell::new(callback)),
            )
        };

        // This must be done before `NSApp()` (equivalent to sending
        // `sharedApplication`) is called anywhere else, or we'll end up
        // with the wrong `NSApplication` class and the wrong thread could
        // be marked as main.
        let ns_app: id = unsafe { msg_send![APP_CLASS.0, sharedApplication] };

        let mut app_delegate_state = Box::pin(AppDelegateState::new(callback.clone()));
        let app_delegate_state_ptr =
            unsafe { app_delegate_state.as_mut().get_unchecked_mut() as *mut _ };
        let ns_app_delegate_alloc: id = unsafe { msg_send![APP_DELEGATE_CLASS.0, alloc] };
        let ns_app_delegate: id =
            unsafe { msg_send![ns_app_delegate_alloc, initWithState: app_delegate_state_ptr] };
        autoreleasepool(|| {
            let _: () = unsafe { msg_send![ns_app, setDelegate: ns_app_delegate] };
        });

        MainLoopState {
            ns_app: unsafe { NSApp() },
            ns_app_delegate,
            app_delegate_state,
            callback,
            redraw_pending: vec![],
        }
    }
}
