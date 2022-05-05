use crate::platform::platform::surface::delegate_state::DelegateState;
use cocoa::appkit::NSWindow;
use cocoa::base::{id, nil};
use cocoa::foundation::{NSSize, NSUInteger};
use objc::declare::ClassDecl;
use objc::runtime::{Class, Object, Sel, BOOL};
use std::os::raw::c_void;
use std::sync::atomic::{AtomicUsize, Ordering};

pub struct DelegateClass(pub &'static Class);
impl DelegateClass {
    const STATE_IVAR_NAME: &'static str = "_state";

    pub fn new() -> Self {
        static IDS: AtomicUsize = AtomicUsize::new(0);
        let id = IDS.fetch_add(1, Ordering::Acquire);
        let class = unsafe {
            let superclass = class!(NSResponder);
            let mut decl =
                ClassDecl::new(format!("WindowDelegate{}", id).as_str(), superclass).unwrap();

            decl.add_method(
                sel!(dealloc),
                Self::dealloc as extern "C" fn(&mut Object, Sel),
            );
            decl.add_method(
                sel!(initWithState:),
                Self::init_with_state as extern "C" fn(&mut Object, Sel, *mut c_void) -> id,
            );
            decl.add_method(
                sel!(windowShouldClose:),
                Self::window_should_close as extern "C" fn(&mut Object, Sel, id) -> BOOL,
            );
            decl.add_method(
                sel!(windowWillClose:),
                Self::window_will_close as extern "C" fn(&mut Object, Sel, id),
            );
            decl.add_method(
                sel!(windowDidResize:),
                Self::window_did_resize as extern "C" fn(&mut Object, Sel, id),
            );
            decl.add_method(
                sel!(windowWillResize:toSize:),
                Self::window_will_resize_to_size
                    as extern "C" fn(&mut Object, Sel, id, NSSize) -> NSSize,
            );
            decl.add_method(
                sel!(windowDidMove:),
                Self::window_did_move as extern "C" fn(&mut Object, Sel, id),
            );
            decl.add_method(
                sel!(windowDidChangeBackingProperties:),
                Self::window_did_change_backing_properties as extern "C" fn(&mut Object, Sel, id),
            );
            decl.add_method(
                sel!(windowDidBecomeKey:),
                Self::window_did_become_key as extern "C" fn(&mut Object, Sel, id),
            );
            decl.add_method(
                sel!(windowDidResignKey:),
                Self::window_did_resign_key as extern "C" fn(&mut Object, Sel, id),
            );
            decl.add_method(
                sel!(draggingEntered:),
                Self::dragging_entered as extern "C" fn(&mut Object, Sel, id) -> BOOL,
            );
            decl.add_method(
                sel!(prepareForDragOperation:),
                Self::prepare_for_drag_operation as extern "C" fn(&mut Object, Sel, id) -> BOOL,
            );
            decl.add_method(
                sel!(performDragOperation:),
                Self::perform_drag_operation as extern "C" fn(&mut Object, Sel, id) -> BOOL,
            );
            decl.add_method(
                sel!(concludeDragOperation:),
                Self::conclude_drag_operation as extern "C" fn(&mut Object, Sel, id),
            );
            decl.add_method(
                sel!(draggingExited:),
                Self::dragging_exited as extern "C" fn(&mut Object, Sel, id),
            );

            decl.add_method(
                sel!(window:willUseFullScreenPresentationOptions:),
                Self::window_will_use_fullscreen_presentation_options
                    as extern "C" fn(&mut Object, Sel, id, NSUInteger) -> NSUInteger,
            );
            decl.add_method(
                sel!(windowDidEnterFullScreen:),
                Self::window_did_enter_fullscreen as extern "C" fn(&mut Object, Sel, id),
            );
            decl.add_method(
                sel!(windowWillEnterFullScreen:),
                Self::window_will_enter_fullscreen as extern "C" fn(&mut Object, Sel, id),
            );
            decl.add_method(
                sel!(windowDidExitFullScreen:),
                Self::window_did_exit_fullscreen as extern "C" fn(&mut Object, Sel, id),
            );
            decl.add_method(
                sel!(windowWillExitFullScreen:),
                Self::window_will_exit_fullscreen as extern "C" fn(&mut Object, Sel, id),
            );
            decl.add_method(
                sel!(windowDidFailToEnterFullScreen:),
                Self::window_did_fail_to_enter_fullscreen as extern "C" fn(&mut Object, Sel, id),
            );
            decl.add_ivar::<*mut c_void>(Self::STATE_IVAR_NAME);
            decl.register()
        };
        DelegateClass(class)
    }

    unsafe fn cast_state_mut(state: &mut *mut c_void) -> &mut DelegateState {
        &mut *(*state as *mut DelegateState)
    }

    unsafe fn get_state_mut(object: &mut Object) -> &mut DelegateState {
        Self::cast_state_mut(object.get_mut_ivar(Self::STATE_IVAR_NAME))
    }

    extern "C" fn dealloc(this: &mut Object, _sel: Sel) {
        // Drop the state value which was previously allocated on the heap
        unsafe { (this as id).setDelegate_(nil) };
        //unsafe { Box::from_raw(Self::get_state_mut(this) as *mut DelegateState); };
    }

    extern "C" fn init_with_state(this: &mut Object, _sel: Sel, state: *mut c_void) -> id {
        unsafe {
            let this: id = msg_send![this, init];
            if this != nil {
                (*this).set_ivar(Self::STATE_IVAR_NAME, state);
            }
            this
        }
    }

    extern "C" fn window_should_close(this: &mut Object, _: Sel, _: id) -> BOOL {
        unsafe { Self::get_state_mut(this) }
            .window_should_close()
            .into()
    }

    extern "C" fn window_will_close(this: &mut Object, _: Sel, _: id) {
        unsafe { Self::get_state_mut(this) }.window_will_close();
    }

    extern "C" fn window_did_resize(this: &mut Object, _: Sel, _: id) {
        unsafe { Self::get_state_mut(this) }.window_did_resize();
    }

    extern "C" fn window_will_resize_to_size(
        this: &mut Object,
        _: Sel,
        _: id,
        size: NSSize,
    ) -> NSSize {
        unsafe { Self::get_state_mut(this) }.window_will_resize_to_size(size);
        size
    }

    extern "C" fn window_did_move(this: &mut Object, _: Sel, _: id) {
        unsafe { Self::get_state_mut(this) }.window_did_move();
    }

    extern "C" fn window_did_change_backing_properties(this: &mut Object, _: Sel, _: id) {
        unsafe { Self::get_state_mut(this) }.window_did_change_backing_properties()
    }

    extern "C" fn window_did_become_key(this: &mut Object, _: Sel, _: id) {
        unsafe { Self::get_state_mut(this) }.window_did_become_key();
    }

    extern "C" fn window_did_resign_key(this: &mut Object, _: Sel, _: id) {
        unsafe { Self::get_state_mut(this) }.window_did_resign_key();
    }

    extern "C" fn dragging_entered(this: &mut Object, _: Sel, sender: id) -> BOOL {
        unsafe { Self::get_state_mut(this) }
            .dragging_entered()
            .into()
    }

    extern "C" fn prepare_for_drag_operation(this: &mut Object, _: Sel, _: id) -> BOOL {
        unsafe { Self::get_state_mut(this) }
            .prepare_for_drag_operation()
            .into()
    }

    extern "C" fn perform_drag_operation(this: &mut Object, _: Sel, sender: id) -> BOOL {
        unsafe { Self::get_state_mut(this) }
            .perform_drag_operation()
            .into()
    }

    extern "C" fn conclude_drag_operation(this: &mut Object, _: Sel, _: id) {
        unsafe { Self::get_state_mut(this) }.conclude_drag_operation();
    }

    extern "C" fn dragging_exited(this: &mut Object, _: Sel, _: id) {
        unsafe { Self::get_state_mut(this) }.dragging_exited();
    }

    extern "C" fn window_will_enter_fullscreen(this: &mut Object, _: Sel, _: id) {
        unsafe { Self::get_state_mut(this) }.window_will_enter_fullscreen();
    }

    extern "C" fn window_will_exit_fullscreen(this: &mut Object, _: Sel, _: id) {
        unsafe { Self::get_state_mut(this) }.window_will_exit_fullscreen();
    }

    extern "C" fn window_will_use_fullscreen_presentation_options(
        this: &mut Object,
        _: Sel,
        _: id,
        proposed_options: NSUInteger,
    ) -> NSUInteger {
        unsafe { Self::get_state_mut(this) }
            .window_will_use_fullscreen_presentation_options(proposed_options)
    }

    extern "C" fn window_did_enter_fullscreen(this: &mut Object, _: Sel, _: id) {
        unsafe { Self::get_state_mut(this) }.window_did_enter_fullscreen();
    }

    extern "C" fn window_did_exit_fullscreen(this: &mut Object, _: Sel, _: id) {
        unsafe { Self::get_state_mut(this) }.window_did_exit_fullscreen();
    }

    extern "C" fn window_did_fail_to_enter_fullscreen(this: &mut Object, _: Sel, _: id) {
        unsafe { Self::get_state_mut(this) }.window_did_fail_to_enter_fullscreen();
    }
}
