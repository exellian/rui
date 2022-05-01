use cocoa::appkit::{NSView, NSWindow};
use cocoa::base::id;
use cocoa::foundation::NSUInteger;
use objc::runtime::{BOOL, NO, YES};
use rui_util::Extent;
use crate::event::{Event, LoopTarget};
use crate::platform::platform::util;
use crate::surface::{SurfaceEvent, SurfaceId};

pub struct DelegateState<'main, 'child> {
    ns_window: id,
    ns_view: id,
    loop_target: LoopTarget<'main, 'child>
}
impl<'main, 'child> DelegateState<'main, 'child> {

    pub fn new(
        ns_window: id,
        ns_view: id,
        loop_target: LoopTarget<'main, 'child>
    ) -> Self {
        DelegateState {
            ns_window,
            ns_view,
            loop_target
        }
    }

    fn get_scale_factor(&self) -> f64 {
        (unsafe { NSWindow::backingScaleFactor(self.ns_window) }) as f64
    }

    pub fn window_should_close(&mut self) -> BOOL {
        NO
    }

    pub fn window_will_close(&mut self) {

    }

    pub fn window_did_resize(&mut self) {
        let rect = unsafe { NSView::frame(self.ns_view) };
        let scale_factor = self.get_scale_factor();
        let size = (rect.size.width as f64 * scale_factor, rect.size.height as f64 * scale_factor);
        util::queue_event(&self.loop_target, Event::SurfaceEvent {
            id: SurfaceId::from(0),//self.window.id, // TODO window id
            event: SurfaceEvent::Resized(Extent {
                width: size.0.round() as u32,
                height: size.1.round() as u32
            })
        });
    }

    pub fn window_did_move(&mut self) {

    }

    pub fn window_did_change_backing_properties(&mut self) {

    }

    pub fn window_did_become_key(&mut self) {

    }

    pub fn window_did_resign_key(&mut self) {

    }

    pub fn dragging_entered(&mut self) -> BOOL {
        YES
    }

    pub fn prepare_for_drag_operation(&mut self) -> BOOL {
        YES
    }

    pub fn perform_drag_operation(&mut self) -> BOOL {
        YES
    }

    pub fn conclude_drag_operation(&mut self) {
    }

    pub fn dragging_exited(&mut self) {

    }

    pub fn window_will_enter_fullscreen(&mut self) {

    }

    pub fn window_will_exit_fullscreen(&mut self) {

    }

    pub fn window_will_use_fullscreen_presentation_options(
        &mut self,
        proposed_options: NSUInteger,
    ) -> NSUInteger {
        let mut options: NSUInteger = proposed_options;

        options
    }

    pub fn window_did_enter_fullscreen(&mut self) {

    }

    pub fn window_did_exit_fullscreen(&mut self) {

    }

    pub fn window_did_fail_to_enter_fullscreen(&mut self) {

    }
}