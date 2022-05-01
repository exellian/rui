use std::cell::RefCell;
use std::collections::VecDeque;
use std::mem;
use cocoa::base::{id, nil};
use objc::rc::autoreleasepool;
use cocoa::appkit::{NSApp, NSEventMask};
use cocoa::foundation::NSDefaultRunLoopMode;
use objc::runtime::YES;
use crate::event::{Event, Flow, InnerLoop};

pub struct MainLoop {
    app: id,
    queue: RefCell<VecDeque<Event>>
}
impl MainLoop {

    pub fn new() -> Self {


        MainLoop {
            app: unsafe { NSApp() },
            queue: RefCell::new(VecDeque::new())
        }
    }

    pub(crate) fn queue_event(&self, event: Event) {
        self.queue.borrow_mut().push_back(event);
    }
}
impl InnerLoop for MainLoop {

    fn wake_up(&self) {
        todo!()
    }

    fn process(&self, flow: &Flow) -> VecDeque<Event> {

        // This block will try to receive the next event from the event queue.
        // The event (NSEvent) gets then propagated through the application by calling sendEvent: event
        // After the call the magic happens and the own event queue gets filled with events:
        //  This works because sendEvent triggers a lot of callbacks that have a reference to this event loop
        //  and therefore they can push events to the queue.
        autoreleasepool(|| unsafe {
            let until_date: id = match flow {
                Flow::Wait => msg_send![class!(NSDate), distantFuture],
                Flow::Poll => nil, // See https://github.com/exellian/rui/issues/28#issuecomment-1109153317
                _ => unreachable!()
            };
            let event: id = msg_send![
                self.app,
                nextEventMatchingMask:(NSEventMask::NSAnyEventMask)
                untilDate:until_date
                inMode:NSDefaultRunLoopMode
                dequeue:YES
            ];
            if event != nil {
                let _: () = msg_send![self.app, sendEvent:event];
            }
        });
        // Take the events in the event queue and return them
        mem::take(&mut *self.queue.borrow_mut())
    }
}