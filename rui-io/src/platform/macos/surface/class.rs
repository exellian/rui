use std::sync::atomic::{AtomicUsize, Ordering};
use objc::declare::ClassDecl;
use objc::runtime::{BOOL, Object, Sel, YES};

pub struct Class(&'static objc::runtime::Class);
impl Class {

    pub fn new() -> Self {
        static IDS: AtomicUsize = AtomicUsize::new(0);
        let id = IDS.fetch_add(1, Ordering::Acquire);
        let class = unsafe {
            let window_superclass = class!(NSWindow);
            let mut decl = ClassDecl::new(format!("Window{}", id).as_str(), window_superclass).unwrap();
            decl.add_method(
                sel!(canBecomeMainWindow),
                Self::can_become_main_window as extern "C" fn(&mut Object, Sel) -> BOOL,
            );
            decl.add_method(
                sel!(canBecomeKeyWindow),
                Self::can_become_key_window as extern "C" fn(&mut Object, Sel) -> BOOL,
            );
            decl.register()
        };
        Class(class)
    }

    pub fn as_objc_class(&self) -> &'static objc::runtime::Class {
        &self.0
    }

    pub extern "C" fn can_become_main_window(this: &mut Object, _: Sel) -> BOOL {
        YES
    }

    pub extern "C" fn can_become_key_window(this: &mut Object, _: Sel) -> BOOL {
        YES
    }
}