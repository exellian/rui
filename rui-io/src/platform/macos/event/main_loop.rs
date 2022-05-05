use cocoa::appkit::{NSApplication, NSEventMask};
use cocoa::base::{id, nil};
use cocoa::foundation::NSDefaultRunLoopMode;
use core_foundation::runloop::{CFRunLoopGetMain, CFRunLoopWakeUp};
use objc::rc::autoreleasepool;
use objc::runtime::{NO, YES};

use crate::event::{Event, Flow, InnerLoop};
use crate::platform::event::main_loop_state::MainLoopState;
use crate::platform::event::Queue;
use crate::surface::SurfaceEvent;

pub struct MainLoop {
    state: Option<MainLoopState>,
}

impl MainLoop {
    pub fn new() -> Self {
        MainLoop { state: None }
    }

    pub fn state(&self) -> &MainLoopState {
        self.state.as_ref().unwrap()
    }
    pub fn state_mut(&mut self) -> &mut MainLoopState {
        self.state.as_mut().unwrap()
    }
}

impl InnerLoop for MainLoop {
    type Queue = Queue;

    fn wake_up(&self) {
        unsafe { CFRunLoopWakeUp(CFRunLoopGetMain()) };
    }

    fn init(&mut self, callback: impl FnMut(&Event)) {
        self.state = Some(MainLoopState::new(callback));
        autoreleasepool(|| unsafe {
            let _: () = msg_send![self.state().ns_app, finishLaunching];
        });
    }

    fn process(&mut self, flow: &Flow) {
        let state = self.state_mut();
        // This block will try to receive the next event from the event queue.
        // The event (NSEvent) gets then propagated through the application by calling sendEvent: event
        // After the call the magic happens and the own event queue gets filled with events:
        //  This works because sendEvent triggers a lot of callbacks that have a reference to this event loop
        //  and therefore they can push events to the queue.
        autoreleasepool(|| unsafe {
            let event: id = state
                .ns_app
                .nextEventMatchingMask_untilDate_inMode_dequeue_(
                    NSEventMask::NSAnyEventMask.bits(),
                    nil,
                    NSDefaultRunLoopMode,
                    YES,
                );
            state.ns_app.sendEvent_(event);
            if let Flow::Wait = flow {
                let _: id = state
                    .ns_app
                    .nextEventMatchingMask_untilDate_inMode_dequeue_(
                        NSEventMask::NSAnyEventMask.bits(),
                        msg_send![class!(NSDate), distantFuture],
                        NSDefaultRunLoopMode,
                        NO,
                    );
            }
        });
        //println!("loop end");
        if !state.redraw_pending.is_empty() {
            for id in state.redraw_pending.drain(..) {
                (state.callback.as_ref().borrow_mut())(&Event::SurfaceEvent {
                    id,
                    event: SurfaceEvent::Redraw,
                });
            }
        }
    }
}
