use cocoa::base::{id, nil};
use objc::rc::autoreleasepool;
use cocoa::appkit::{NSApp, NSEventMask};
use cocoa::foundation::NSDefaultRunLoopMode;
use objc::runtime::YES;
use crate::event::{Event, Flow, InnerLoop};

pub struct MainLoop {
    app: id
}
impl MainLoop {

    pub fn new() -> Self {
        MainLoop {
            app: unsafe { NSApp() }
        }
    }
}
impl InnerLoop for MainLoop {

    fn wake_up(&self) {
        todo!()
    }

    fn process(&self, flow: &Flow, callback: impl FnMut(Option<&Event>)) {
        let callback_ref = &callback as &dyn FnMut(Option<&Event>);
        let mut called = false;
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
        if !called {
            callback(None);
        }
    }
}