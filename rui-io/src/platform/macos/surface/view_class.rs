use std::cell::UnsafeCell;
use std::mem::MaybeUninit;
use std::os::raw::c_void;
use std::sync::atomic::{AtomicUsize, Ordering};

use cocoa::appkit::NSTabView;
use cocoa::base::{id, nil, YES};
use cocoa::foundation::{NSPoint, NSRect, NSString, NSUInteger};
use objc::declare::ClassDecl;
use objc::runtime::{Class, Object, Protocol, Sel, BOOL};

use crate::platform::platform::ffi::{NSMutableAttributedString, NSRange};
use crate::platform::platform::surface::view_state::ViewState;
use crate::platform::platform::util;

pub struct ViewClass(pub &'static Class);
impl ViewClass {
    const STATE_IVAR_NAME: &'static str = "_state";

    pub fn new() -> Self {
        static IDS: AtomicUsize = AtomicUsize::new(0);
        let id = IDS.fetch_add(1, Ordering::Acquire);
        let class = unsafe {
            let superclass = class!(NSView);
            let mut decl = ClassDecl::new(format!("View{}", id).as_str(), superclass).unwrap();
            decl.add_method(
                sel!(dealloc),
                Self::dealloc as extern "C" fn(&mut Object, Sel),
            );
            decl.add_method(
                sel!(initWithState:),
                Self::init_with_state as extern "C" fn(&mut Object, Sel, *mut c_void) -> id,
            );
            decl.add_method(
                sel!(viewDidMoveToWindow),
                Self::view_did_move_to_window as extern "C" fn(&mut Object, Sel),
            );
            decl.add_method(
                sel!(drawRect:),
                Self::draw_rect as extern "C" fn(&mut Object, Sel, NSRect),
            );
            decl.add_method(
                sel!(acceptsFirstResponder),
                Self::accepts_first_responder as extern "C" fn(&mut Object, Sel) -> BOOL,
            );
            decl.add_method(
                sel!(touchBar),
                Self::touch_bar as extern "C" fn(&mut Object, Sel) -> BOOL,
            );
            decl.add_method(
                sel!(resetCursorRects),
                Self::reset_cursor_rects as extern "C" fn(&mut Object, Sel),
            );
            decl.add_method(
                sel!(hasMarkedText),
                Self::has_marked_text as extern "C" fn(&mut Object, Sel) -> BOOL,
            );
            decl.add_method(
                sel!(markedRange),
                Self::marked_range as extern "C" fn(&mut Object, Sel) -> NSRange,
            );
            decl.add_method(
                sel!(selectedRange),
                Self::selected_range as extern "C" fn(&mut Object, Sel) -> NSRange,
            );
            decl.add_method(
                sel!(setMarkedText:selectedRange:replacementRange:),
                Self::set_marked_text as extern "C" fn(&mut Object, Sel, id, NSRange, NSRange),
            );
            decl.add_method(
                sel!(unmarkText),
                Self::unmark_text as extern "C" fn(&mut Object, Sel),
            );
            decl.add_method(
                sel!(validAttributesForMarkedText),
                Self::valid_attributes_for_marked_text as extern "C" fn(&mut Object, Sel) -> id,
            );
            decl.add_method(
                sel!(attributedSubstringForProposedRange:actualRange:),
                Self::attributed_substring_for_proposed_range
                    as extern "C" fn(&mut Object, Sel, NSRange, *mut c_void) -> id,
            );
            decl.add_method(
                sel!(insertText:replacementRange:),
                Self::insert_text as extern "C" fn(&mut Object, Sel, id, NSRange),
            );
            decl.add_method(
                sel!(characterIndexForPoint:),
                Self::character_index_for_point
                    as extern "C" fn(&mut Object, Sel, NSPoint) -> NSUInteger,
            );
            decl.add_method(
                sel!(firstRectForCharacterRange:actualRange:),
                Self::first_rect_for_character_range
                    as extern "C" fn(&mut Object, Sel, NSRange, *mut c_void) -> NSRect,
            );
            decl.add_method(
                sel!(doCommandBySelector:),
                Self::do_command_by_selector as extern "C" fn(&mut Object, Sel, Sel),
            );
            decl.add_method(
                sel!(keyDown:),
                Self::key_down as extern "C" fn(&mut Object, Sel, id),
            );
            decl.add_method(
                sel!(keyUp:),
                Self::key_up as extern "C" fn(&mut Object, Sel, id),
            );
            decl.add_method(
                sel!(flagsChanged:),
                Self::flags_changed as extern "C" fn(&mut Object, Sel, id),
            );
            decl.add_method(
                sel!(insertTab:),
                Self::insert_tab as extern "C" fn(&mut Object, Sel, id),
            );
            decl.add_method(
                sel!(insertBackTab:),
                Self::insert_back_tab as extern "C" fn(&mut Object, Sel, id),
            );
            decl.add_method(
                sel!(mouseDown:),
                Self::mouse_down as extern "C" fn(&mut Object, Sel, id),
            );
            decl.add_method(
                sel!(mouseUp:),
                Self::mouse_up as extern "C" fn(&mut Object, Sel, id),
            );
            decl.add_method(
                sel!(rightMouseDown:),
                Self::right_mouse_down as extern "C" fn(&mut Object, Sel, id),
            );
            decl.add_method(
                sel!(rightMouseUp:),
                Self::right_mouse_up as extern "C" fn(&mut Object, Sel, id),
            );
            decl.add_method(
                sel!(otherMouseDown:),
                Self::other_mouse_down as extern "C" fn(&mut Object, Sel, id),
            );
            decl.add_method(
                sel!(otherMouseUp:),
                Self::other_mouse_up as extern "C" fn(&mut Object, Sel, id),
            );
            decl.add_method(
                sel!(mouseMoved:),
                Self::mouse_moved as extern "C" fn(&mut Object, Sel, id),
            );
            decl.add_method(
                sel!(mouseDragged:),
                Self::mouse_dragged as extern "C" fn(&mut Object, Sel, id),
            );
            decl.add_method(
                sel!(rightMouseDragged:),
                Self::right_mouse_dragged as extern "C" fn(&mut Object, Sel, id),
            );
            decl.add_method(
                sel!(otherMouseDragged:),
                Self::other_mouse_dragged as extern "C" fn(&mut Object, Sel, id),
            );
            decl.add_method(
                sel!(mouseEntered:),
                Self::mouse_entered as extern "C" fn(&mut Object, Sel, id),
            );
            decl.add_method(
                sel!(mouseExited:),
                Self::mouse_exited as extern "C" fn(&mut Object, Sel, id),
            );
            decl.add_method(
                sel!(scrollWheel:),
                Self::scroll_wheel as extern "C" fn(&mut Object, Sel, id),
            );
            decl.add_method(
                sel!(pressureChangeWithEvent:),
                Self::pressure_change_with_event as extern "C" fn(&mut Object, Sel, id),
            );
            decl.add_method(
                sel!(_wantsKeyDownForEvent:),
                Self::wants_key_down_for_event as extern "C" fn(&mut Object, Sel, id) -> BOOL,
            );
            decl.add_method(
                sel!(cancelOperation:),
                Self::cancel_operation as extern "C" fn(&mut Object, Sel, id),
            );
            decl.add_method(
                sel!(frameDidChange:),
                Self::frame_did_change as extern "C" fn(&mut Object, Sel, id),
            );
            decl.add_method(
                sel!(acceptsFirstMouse:),
                Self::accepts_first_mouse as extern "C" fn(&mut Object, Sel, id) -> BOOL,
            );
            decl.add_ivar::<*mut c_void>(Self::STATE_IVAR_NAME);
            decl.add_ivar::<id>("markedText");
            let protocol = Protocol::get("NSTextInputClient").unwrap();
            decl.add_protocol(protocol);
            decl.register()
        };
        ViewClass(class)
    }

    // borrows individual fields of object as mut

    fn get_fields_mut<'a, const SIZE: usize>(
        object: &'a mut Object,
        fields: [&str; SIZE],
    ) -> [&'a mut id; SIZE] {
        let object_ref_cell = UnsafeCell::new(object);
        let mut rets: [MaybeUninit<&mut id>; SIZE] = unsafe { MaybeUninit::uninit().assume_init() };
        for i in 0..SIZE {
            let mut_ref: &mut Object = unsafe { *object_ref_cell.get() as &mut _ };
            rets[i] = MaybeUninit::new(unsafe { mut_ref.get_mut_ivar(fields[i]) });
        }
        rets.map(|v| unsafe { v.assume_init() })
    }

    unsafe fn cast_state_mut(state: &mut *mut c_void) -> &mut ViewState {
        &mut *(*state as *mut ViewState)
    }

    unsafe fn get_state_mut(object: &mut Object) -> &mut ViewState {
        Self::cast_state_mut(object.get_mut_ivar(Self::STATE_IVAR_NAME))
    }

    /// User must ensure that the fields exist and that no field gets borrowed twice or more times.
    unsafe fn get_state_and_fields_mut<'a, const SIZE: usize>(
        object: &'a mut Object,
        fields: [&str; SIZE],
    ) -> (&'a mut ViewState, [&'a mut id; SIZE]) {
        let object_ref_cell = UnsafeCell::new(object);
        let state = Self::get_state_mut(*object_ref_cell.get() as &mut _);
        let fields_mut =
            Self::get_fields_mut(unsafe { *object_ref_cell.get() as &mut Object }, fields);
        (state, fields_mut)
    }

    extern "C" fn dealloc(this: &mut Object, _sel: Sel) {
        unsafe {
            let marked_text: id = *this.get_ivar("markedText");
            let _: () = msg_send![marked_text, release];
            unsafe { (this as id).setDelegate_(nil) };
            // drop memory of heap
            //Box::from_raw(state as *mut ViewState);
        }
    }

    extern "C" fn init_with_state(this: &mut Object, _sel: Sel, state: *mut c_void) -> id {
        unsafe {
            let this: id = msg_send![this, init];
            if this != nil {
                (*this).set_ivar(Self::STATE_IVAR_NAME, state);
                let marked_text =
                    NSMutableAttributedString::init(NSMutableAttributedString::alloc(nil));
                (*this).set_ivar("markedText", marked_text);
                let _: () = msg_send![this, setPostsFrameChangedNotifications: YES];

                let notification_center: &mut Object =
                    msg_send![class!(NSNotificationCenter), defaultCenter];
                let notification_name =
                    NSString::alloc(nil).init_str("NSViewFrameDidChangeNotification");
                let _: () = msg_send![
                    notification_center,
                    addObserver: this
                    selector: sel!(frameDidChange:)
                    name: notification_name
                    object: this
                ];
            }
            this
        }
    }

    extern "C" fn view_did_move_to_window(this: &mut Object, _sel: Sel) {
        unsafe { Self::get_state_mut(this) }.view_did_move_to_window()
    }

    extern "C" fn frame_did_change(this: &mut Object, _sel: Sel, _event: id) {
        unsafe { Self::get_state_mut(this) }.frame_did_change()
    }

    extern "C" fn draw_rect(this: &mut Object, _sel: Sel, rect: NSRect) {
        unsafe { Self::get_state_mut(this) }.draw_rect(rect);
        unsafe {
            let superclass = util::superclass(this);
            let () = msg_send![super(this, superclass), drawRect: rect];
        }
    }

    extern "C" fn accepts_first_responder(this: &mut Object, _sel: Sel) -> BOOL {
        unsafe { Self::get_state_mut(this) }.accepts_first_responder()
    }

    extern "C" fn touch_bar(this: &mut Object, _sel: Sel) -> BOOL {
        unsafe { Self::get_state_mut(this) }.touch_bar()
    }

    extern "C" fn reset_cursor_rects(this: &mut Object, _sel: Sel) {
        unsafe { Self::get_state_mut(this) }.reset_cursor_rects()
    }

    extern "C" fn has_marked_text(this: &mut Object, _sel: Sel) -> BOOL {
        let (state, fields) = unsafe { Self::get_state_and_fields_mut(this, ["markedText"]) };
        state.has_marked_text(*fields[0])
    }

    extern "C" fn marked_range(this: &mut Object, _sel: Sel) -> NSRange {
        let (state, fields) = unsafe { Self::get_state_and_fields_mut(this, ["markedText"]) };
        state.marked_range(*fields[0])
    }

    extern "C" fn selected_range(this: &mut Object, _sel: Sel) -> NSRange {
        unsafe { Self::get_state_mut(this) }.selected_range()
    }

    extern "C" fn set_marked_text(
        this: &mut Object,
        _sel: Sel,
        string: id,
        _selected_range: NSRange,
        _replacement_range: NSRange,
    ) {
        let (state, fields) = unsafe { Self::get_state_and_fields_mut(this, ["markedText"]) };
        state.set_marked_text(fields[0], string);
    }

    extern "C" fn unmark_text(this: &mut Object, _sel: Sel) {
        let (state, fields) = unsafe { Self::get_state_and_fields_mut(this, ["markedText"]) };
        state.unmark_text(*fields[0]);
        let input_context: id = unsafe { msg_send![this, inputContext] };
        let _: () = unsafe { msg_send![input_context, discardMarkedText] };
    }

    extern "C" fn valid_attributes_for_marked_text(this: &mut Object, _sel: Sel) -> id {
        unsafe { Self::get_state_mut(this) }.valid_attributes_for_marked_text()
    }

    extern "C" fn attributed_substring_for_proposed_range(
        this: &mut Object,
        _sel: Sel,
        _range: NSRange,
        _actual_range: *mut c_void, // *mut NSRange
    ) -> id {
        unsafe { Self::get_state_mut(this) }.attributed_substring_for_proposed_range()
    }

    extern "C" fn character_index_for_point(
        this: &mut Object,
        _sel: Sel,
        _point: NSPoint,
    ) -> NSUInteger {
        unsafe { Self::get_state_mut(this) }.character_index_for_point()
    }

    extern "C" fn first_rect_for_character_range(
        this: &mut Object,
        _sel: Sel,
        _range: NSRange,
        _actual_range: *mut c_void, // *mut NSRange
    ) -> NSRect {
        unsafe { Self::get_state_mut(this) }.first_rect_for_character_range()
    }

    extern "C" fn insert_text(
        this: &mut Object,
        _sel: Sel,
        string: id,
        _replacement_range: NSRange,
    ) {
        unsafe { Self::get_state_mut(this) }.insert_text(string)
    }

    extern "C" fn do_command_by_selector(this: &mut Object, _sel: Sel, command: Sel) {
        unsafe { Self::get_state_mut(this) }.do_command_by_selector(command)
    }

    extern "C" fn key_down(this: &mut Object, _sel: Sel, event: id) {
        unsafe { Self::get_state_mut(this) }.key_down(event)
    }

    extern "C" fn key_up(this: &mut Object, _sel: Sel, event: id) {
        unsafe { Self::get_state_mut(this) }.key_up(event)
    }

    extern "C" fn flags_changed(this: &mut Object, _sel: Sel, event: id) {
        unsafe { Self::get_state_mut(this) }.flags_changed(event)
    }

    extern "C" fn insert_tab(this: &mut Object, _sel: Sel, _sender: id) {
        unsafe { Self::get_state_mut(this) }.insert_tab()
    }

    extern "C" fn insert_back_tab(this: &mut Object, _sel: Sel, _sender: id) {
        unsafe { Self::get_state_mut(this) }.insert_back_tab()
    }

    extern "C" fn cancel_operation(this: &mut Object, _sel: Sel, _sender: id) {
        unsafe { Self::get_state_mut(this) }.cancel_operation()
    }

    extern "C" fn mouse_down(this: &mut Object, _sel: Sel, event: id) {
        unsafe { Self::get_state_mut(this) }.mouse_down(event)
    }

    extern "C" fn mouse_up(this: &mut Object, _sel: Sel, event: id) {
        unsafe { Self::get_state_mut(this) }.mouse_up(event)
    }

    extern "C" fn right_mouse_down(this: &mut Object, _sel: Sel, event: id) {
        unsafe { Self::get_state_mut(this) }.right_mouse_down(event)
    }

    extern "C" fn right_mouse_up(this: &mut Object, _sel: Sel, event: id) {
        unsafe { Self::get_state_mut(this) }.right_mouse_up(event)
    }

    extern "C" fn other_mouse_down(this: &mut Object, _sel: Sel, event: id) {
        unsafe { Self::get_state_mut(this) }.other_mouse_down(event)
    }

    extern "C" fn other_mouse_up(this: &mut Object, _sel: Sel, event: id) {
        unsafe { Self::get_state_mut(this) }.other_mouse_up(event)
    }

    extern "C" fn mouse_moved(this: &mut Object, _sel: Sel, event: id) {
        unsafe { Self::get_state_mut(this) }.mouse_moved(event)
    }

    extern "C" fn mouse_dragged(this: &mut Object, _sel: Sel, event: id) {
        unsafe { Self::get_state_mut(this) }.mouse_dragged(event)
    }

    extern "C" fn right_mouse_dragged(this: &mut Object, _sel: Sel, event: id) {
        unsafe { Self::get_state_mut(this) }.right_mouse_dragged(event)
    }

    extern "C" fn other_mouse_dragged(this: &mut Object, _sel: Sel, event: id) {
        unsafe { Self::get_state_mut(this) }.other_mouse_dragged(event)
    }

    extern "C" fn mouse_entered(this: &mut Object, _sel: Sel, _event: id) {
        unsafe { Self::get_state_mut(this) }.mouse_entered()
    }

    extern "C" fn mouse_exited(this: &mut Object, _sel: Sel, _event: id) {
        unsafe { Self::get_state_mut(this) }.mouse_exited()
    }

    extern "C" fn scroll_wheel(this: &mut Object, _sel: Sel, event: id) {
        unsafe { Self::get_state_mut(this) }.scroll_wheel(event)
    }

    extern "C" fn pressure_change_with_event(this: &mut Object, _sel: Sel, event: id) {
        unsafe { Self::get_state_mut(this) }.pressure_change_with_event(event)
    }

    extern "C" fn wants_key_down_for_event(this: &mut Object, _sel: Sel, _event: id) -> BOOL {
        unsafe { Self::get_state_mut(this) }.wants_key_down_for_event()
    }

    extern "C" fn accepts_first_mouse(this: &mut Object, _sel: Sel, _event: id) -> BOOL {
        unsafe { Self::get_state_mut(this) }.accepts_first_mouse()
    }
}
