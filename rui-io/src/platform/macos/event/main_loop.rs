use std::pin::Pin;

use cocoa::appkit::{NSApp, NSApplication, NSEventMask};
use cocoa::base::{id, nil};
use cocoa::foundation::NSDefaultRunLoopMode;
use core_foundation::runloop::{CFRunLoopGetMain, CFRunLoopWakeUp};
use objc::rc::autoreleasepool;
use objc::runtime::{BOOL, NO, YES};
use rui_macros::main;

use crate::event::{Flow, InnerLoop};
use crate::platform::event::app::{AppClass, AppDelegateClass, AppDelegateState};
use crate::platform::event::Queue;

pub struct MainLoop {
    ns_app: id,
    ns_app_delegate: id,
    app_delegate_state: Pin<Box<AppDelegateState>>,
    pub(crate) queue: Queue,
    finished_launching: bool,
}

impl MainLoop {
    pub fn new() -> Self {
        let is_main_thread: BOOL = unsafe { msg_send!(class!(NSThread), isMainThread) };
        if is_main_thread == NO {
            panic!("On macOS, `EventLoop` must be created on the main thread!");
        }

        lazy_static! {
            static ref APP_CLASS: AppClass = AppClass::new();
            static ref APP_DELEGATE_CLASS: AppDelegateClass = AppDelegateClass::new();
        }

        let queue = Queue::new();

        // This must be done before `NSApp()` (equivalent to sending
        // `sharedApplication`) is called anywhere else, or we'll end up
        // with the wrong `NSApplication` class and the wrong thread could
        // be marked as main.
        let ns_app: id = unsafe { msg_send![APP_CLASS.0, sharedApplication] };

        let mut app_delegate_state = Box::pin(AppDelegateState::new(queue.clone()));
        let app_delegate_state_ptr =
            unsafe { app_delegate_state.as_mut().get_unchecked_mut() as *mut _ };
        let ns_app_delegate_alloc: id = unsafe { msg_send![APP_DELEGATE_CLASS.0, alloc] };
        let ns_app_delegate: id =
            unsafe { msg_send![ns_app_delegate_alloc, initWithState: app_delegate_state_ptr] };
        autoreleasepool(|| {
            let _: () = unsafe { msg_send![ns_app, setDelegate: ns_app_delegate] };
        });

        MainLoop {
            ns_app: unsafe { NSApp() },
            ns_app_delegate,
            app_delegate_state,
            queue,
            finished_launching: false,
        }
    }
}

impl InnerLoop for MainLoop {
    type Queue = Queue;

    fn wake_up(&self) {
        unsafe { CFRunLoopWakeUp(CFRunLoopGetMain()) };
    }

    fn process(&mut self, flow: &Flow) -> &mut Queue {
        if !self.finished_launching {
            autoreleasepool(|| unsafe {
                let _: () = msg_send![self.ns_app, finishLaunching];
            });
        }
        // This block will try to receive the next event from the event queue.
        // The event (NSEvent) gets then propagated through the application by calling sendEvent: event
        // After the call the magic happens and the own event queue gets filled with events:
        //  This works because sendEvent triggers a lot of callbacks that have a reference to this event loop
        //  and therefore they can push events to the queue.
        autoreleasepool(|| unsafe {
            let mut until_date: id = match flow {
                Flow::Wait => msg_send![class!(NSDate), distantFuture],
                Flow::Poll => nil, // See https://github.com/exellian/rui/issues/28#issuecomment-1109153317
                _ => unreachable!(),
            };
            if !self.finished_launching {
                until_date = nil;
                self.finished_launching = true;
            }
            let event: id = self.ns_app.nextEventMatchingMask_untilDate_inMode_dequeue_(
                NSEventMask::NSAnyEventMask.bits(),
                until_date,
                NSDefaultRunLoopMode,
                YES,
            );
            if event != nil {
                self.ns_app.sendEvent_(event);
            }
            let _: () = msg_send![self.ns_app, updateWindows];
        });
        &mut self.queue
    }
}
