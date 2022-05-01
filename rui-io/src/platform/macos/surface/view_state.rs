use std::cell::RefCell;
use std::ptr::NonNull;
use std::rc::Rc;
use std::sync::Arc;
use cocoa::appkit::NSWindow;
use cocoa::base::{id, nil};
use cocoa::foundation::{NSPoint, NSRect, NSSize, NSUInteger};
use objc::runtime::{BOOL, NO, Object, Sel, YES};
use crate::event::LoopTarget;
use crate::platform::platform::ffi::{NSMutableAttributedString, NSRange};

pub struct ViewState<'main, 'child> {
    ns_window: id,
    ime_position: (f64, f64),
    loop_target: LoopTarget<'main, 'child>
}
impl<'main, 'child> ViewState<'main, 'child> {

    pub fn new(ns_window: id, loop_target: LoopTarget<'main, 'child>) -> Self {
        ViewState {
            ns_window,
            ime_position: (0.0, 0.0),
            loop_target
        }
    }

    pub fn view_did_move_to_window(&mut self) {

    }

    pub fn frame_did_change(&mut self) {

    }

    pub fn draw_rect(&mut self, rect: NSRect) {

    }

    pub fn accepts_first_responder(&mut self) -> BOOL {
        YES
    }

    pub fn touch_bar(&mut self) -> BOOL {
        NO
    }

    pub fn reset_cursor_rects(&mut self) {

    }

    pub fn has_marked_text(&mut self, marked_text: id) -> BOOL {
        unsafe {
            (marked_text.length() > 0) as BOOL
        }
    }

    pub fn marked_range(&mut self, marked_text: id) -> NSRange {
        let length = unsafe { marked_text.length() };
        if length > 0 {
            NSRange::new(0, length - 1)
        } else {
            NSRange::EMPTY_RANGE
        }
    }

    pub fn selected_range(&mut self) -> NSRange {
        NSRange::EMPTY_RANGE
    }

    pub fn set_marked_text(
        &mut self,
        marked_text_mut: &mut id,
        string: id
    ) {
        let _: () = unsafe { msg_send![(*marked_text_mut), release] };
        let marked_text = unsafe { NSMutableAttributedString::alloc(nil) };
        let has_attr: BOOL = unsafe { msg_send![string, isKindOfClass: class!(NSAttributedString)] };
        if has_attr != NO {
            unsafe { marked_text.initWithAttributedString(string) };
        } else {
            unsafe { marked_text.initWithString(string) };
        };
        *marked_text_mut = marked_text;
    }

    pub fn unmark_text(&mut self, marked_text: id) {
        let mutable_string = unsafe { marked_text.mutableString() };
        let s: id = unsafe { msg_send![class!(NSString), new] };
        let _: () = unsafe { msg_send![mutable_string, setString: s] };
        let _: () = unsafe { msg_send![s, release] };
    }

    pub fn valid_attributes_for_marked_text(&mut self) -> id {
        unsafe { msg_send![class!(NSArray), array] }
    }

    pub fn attributed_substring_for_proposed_range(&mut self) -> id {
        nil
    }

    pub fn character_index_for_point(&mut self) -> NSUInteger {
        0
    }

    pub fn first_rect_for_character_range(&mut self) -> NSRect {
        let frame = unsafe { NSWindow::frame(self.ns_window) };
        let content_rect =
            unsafe { NSWindow::contentRectForFrameRect_(self.ns_window, frame) };
        let base_x = content_rect.origin.x as f64;
        let base_y = (content_rect.origin.y + content_rect.size.height) as f64;
        let x = base_x + self.ime_position.0;
        let y = base_y - self.ime_position.1;
        // TODO check winit
        NSRect::new(NSPoint::new(x as _, y as _), NSSize::new(0.0, 0.0))
    }

    pub fn insert_text(&mut self, string: id) {

    }

    pub fn do_command_by_selector(&mut self, command: Sel) {

    }

    pub fn key_down(&mut self, event: id) {

    }

    pub fn key_up(&mut self, event: id) {

    }

    pub fn flags_changed(&mut self, event: id) {

    }

    pub fn insert_tab(&mut self) {

    }

    pub fn insert_back_tab(&mut self) {

    }

    // Allows us to receive Cmd-. (the shortcut for closing a dialog)
    // https://bugs.eclipse.org/bugs/show_bug.cgi?id=300620#c6
    pub fn cancel_operation(&mut self) {

    }

    pub fn mouse_down(&mut self, event: id) {

    }

    pub fn mouse_up(&mut self, event: id) {

    }

    pub fn right_mouse_down(&mut self, event: id) {

    }

    pub fn right_mouse_up(&mut self, event: id) {

    }

    pub fn other_mouse_down(&mut self, event: id) {

    }

    pub fn other_mouse_up(&mut self, event: id) {

    }

    pub fn mouse_moved(&mut self, event: id) {

    }

    pub fn mouse_dragged(&mut self, event: id) {

    }

    pub fn right_mouse_dragged(&mut self, event: id) {

    }

    pub fn other_mouse_dragged(&mut self, event: id) {

    }

    pub fn mouse_entered(&mut self) {

    }

    pub fn mouse_exited(&mut self) {

    }

    pub fn scroll_wheel(&mut self, event: id) {

    }

    pub fn pressure_change_with_event(&mut self, event: id) {

    }

    // Allows us to receive Ctrl-Tab and Ctrl-Esc.
    // Note that this *doesn't* help with any missing Cmd inputs.
    // https://github.com/chromium/chromium/blob/a86a8a6bcfa438fa3ac2eba6f02b3ad1f8e0756f/ui/views/cocoa/bridged_content_view.mm#L816
    pub fn wants_key_down_for_event(&mut self) -> BOOL {
        YES
    }

    pub fn accepts_first_mouse(&mut self) -> BOOL {
        YES
    }
}