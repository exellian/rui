use std::cell::RefCell;
use std::{mem, ptr};
use std::borrow::BorrowMut;
use std::ptr::NonNull;
use std::rc::Rc;
use cocoa::appkit::{NSApp, NSEventMask};
use cocoa::base::{id, nil};
use cocoa::foundation::NSDefaultRunLoopMode;
use objc::rc::autoreleasepool;
use objc::runtime::{BOOL, NO, YES};
use crate::event::{Event, Flow};

pub struct Loop;
impl Loop {

    pub fn new() -> Self {
        Loop
    }


    /// This function runs the main thread event loop and therefore must be
    /// called from the main thread.
    ///
    /// # Arguments
    /// * callback - This function gets called in every loop iteration. It provides and event if
    ///             an event occurred during this iteration, else None.
    ///             Additionally the user can use the EventLoopTarget to spawn new event loop threads.
    pub fn run<'main, F>(&'main mut self, mut callback: F) -> ! where F: FnMut(&EventLoopTarget, &mut Flow, Option<Event>) {
        let is_main_thread: BOOL = unsafe { msg_send![class!(NSThread), isMainThread] };
        if is_main_thread == NO {
            panic!("Event Loop must be started from the main thread!");
        }
        let callback_ref = &mut callback as &mut dyn FnMut(&EventLoopTarget, &mut Flow, Option<Event>);
        let mut inner_event_loop_target = InnerLoopTarget::new(Flow::Wait, NonNull::from(callback_ref));
        let mut event_loop_target = EventLoopTarget::new(NonNull::from(&inner_event_loop_target));
        inner_event_loop_target.set_outer(NonNull::from(&event_loop_target));

        let app = unsafe { NSApp() };

        loop {
            inner_event_loop_target.reset();
            autoreleasepool(|| unsafe {
                let until_date: id = match *inner_event_loop_target.thread_flow() {
                    Flow::Wait => msg_send![class!(NSDate), distantFuture],
                    Flow::Poll => nil // See https://github.com/exellian/rui/issues/28#issuecomment-1109153317
                };

                let event: id = msg_send![
                    app,
                    nextEventMatchingMask:(NSEventMask::NSAnyEventMask)
                    untilDate:until_date
                    inMode:NSDefaultRunLoopMode
                    dequeue:YES
                ];
                if event != nil {
                    let _: () = msg_send![app, sendEvent:event];
                }
            });
            // If no event got emitted then call the callback with None as event
            if !inner_event_loop_target.emitted() {
                unsafe { inner_event_loop_target.call(None) };
            }
        }
    }
}