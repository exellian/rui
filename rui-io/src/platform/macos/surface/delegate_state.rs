use cocoa::appkit::{NSView, NSWindow};
use cocoa::base::id;
use cocoa::foundation::{NSSize, NSUInteger};
use objc::runtime::{BOOL, NO, YES};
use std::borrow::BorrowMut;
use std::cell::RefCell;
use std::rc::Rc;

use rui_util::Extent;

use crate::event::queue::Enqueue;
use crate::event::{Event, LoopTarget};
use crate::platform::event::Queue;
use crate::platform::platform::util;
use crate::surface::{SurfaceEvent, SurfaceId};

pub struct DelegateState {
    ns_window: id,
    ns_view: id,
    emit_resize: bool,
    callback: Rc<RefCell<dyn FnMut(&Event)>>,
}
impl DelegateState {
    pub fn new(ns_window: id, ns_view: id, callback: Rc<RefCell<dyn FnMut(&Event)>>) -> Self {
        DelegateState {
            ns_window,
            ns_view,
            emit_resize: false,
            callback,
        }
    }

    fn get_scale_factor(&self) -> f64 {
        (unsafe { NSWindow::backingScaleFactor(self.ns_window) }) as f64
    }

    pub fn window_should_close(&mut self) -> BOOL {
        NO
    }

    pub fn window_will_close(&mut self) {}

    fn get_size(&mut self, input: NSSize) -> (u32, u32) {
        let scale_factor = self.get_scale_factor();
        let size = (
            input.width as f64 * scale_factor,
            input.height as f64 * scale_factor,
        );
        (size.0.round() as u32, size.1.round() as u32)
    }

    pub fn window_did_resize(&mut self) {
        if self.emit_resize {
            let rect = unsafe { NSView::frame(self.ns_view) };
            let size = self.get_size(rect.size);
            (self.callback.as_ref().borrow_mut())(&Event::SurfaceEvent {
                id: util::get_window_id(self.ns_window),
                event: SurfaceEvent::Resized(Extent {
                    width: size.0,
                    height: size.1,
                }),
            });
            println!("After");
        }
    }

    pub fn window_will_resize_to_size(&mut self, mut to_size: NSSize) -> NSSize {
        let current = unsafe { NSView::frame(self.ns_view) };

        let to = self.get_size(current.size);
        (self.callback.as_ref().borrow_mut())(&Event::SurfaceEvent {
            id: util::get_window_id(self.ns_window),
            event: SurfaceEvent::Resized(Extent {
                width: to.0,
                height: to.1,
            }),
        });
        self.emit_resize = false;
        to_size
    }

    pub fn window_did_move(&mut self) {}

    pub fn window_did_change_backing_properties(&mut self) {}

    pub fn window_did_become_key(&mut self) {}

    pub fn window_did_resign_key(&mut self) {}

    pub fn dragging_entered(&mut self) -> BOOL {
        YES
    }

    pub fn prepare_for_drag_operation(&mut self) -> BOOL {
        YES
    }

    pub fn perform_drag_operation(&mut self) -> BOOL {
        YES
    }

    pub fn conclude_drag_operation(&mut self) {}

    pub fn dragging_exited(&mut self) {}

    pub fn window_will_enter_fullscreen(&mut self) {}

    pub fn window_will_exit_fullscreen(&mut self) {}

    pub fn window_will_use_fullscreen_presentation_options(
        &mut self,
        proposed_options: NSUInteger,
    ) -> NSUInteger {
        let mut options: NSUInteger = proposed_options;

        options
    }

    pub fn window_did_enter_fullscreen(&mut self) {}

    pub fn window_did_exit_fullscreen(&mut self) {}

    pub fn window_did_fail_to_enter_fullscreen(&mut self) {}
}
