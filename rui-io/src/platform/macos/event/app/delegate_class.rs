use std::os::raw::c_void;
use std::sync::atomic::{AtomicUsize, Ordering};

use cocoa::base::{id, nil};
use objc::declare::ClassDecl;
use objc::runtime::{Class, Object, Sel};

use crate::platform::event::app::delegate_state::DelegateState;

pub struct DelegateClass(pub &'static Class);
impl DelegateClass {
    const STATE_IVAR_NAME: &'static str = "_state";

    pub fn new() -> Self {
        static IDS: AtomicUsize = AtomicUsize::new(0);
        let id = IDS.fetch_add(1, Ordering::Acquire);
        let class = unsafe {
            let superclass = class!(NSResponder);
            let mut decl =
                ClassDecl::new(format!("AppDelegate{}", id).as_str(), superclass).unwrap();

            decl.add_method(
                sel!(initWithState:),
                Self::init_with_state as extern "C" fn(&mut Object, Sel, *mut c_void) -> id,
            );
            decl.add_method(
                sel!(dealloc),
                Self::dealloc as extern "C" fn(&mut Object, Sel),
            );

            decl.add_method(
                sel!(applicationDidFinishLaunching:),
                Self::did_finish_launching as extern "C" fn(&mut Object, Sel, id),
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

    extern "C" fn init_with_state(this: &mut Object, _sel: Sel, state: *mut c_void) -> id {
        unsafe {
            let this: id = msg_send![this, init];
            if this != nil {
                (*this).set_ivar(Self::STATE_IVAR_NAME, state);
            }
            this
        }
    }

    extern "C" fn dealloc(this: &mut Object, _: Sel) {
        //
    }
    extern "C" fn did_finish_launching(this: &mut Object, _: Sel, _: id) {
        unsafe { Self::get_state_mut(this) }.did_finish_launching();
    }
}
