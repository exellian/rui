use std::sync::atomic::{AtomicUsize, Ordering};

use objc::declare::ClassDecl;

pub struct Class(pub &'static objc::runtime::Class);

impl Class {
    const STATE_IVAR_NAME: &'static str = "_state";

    pub fn new() -> Self {
        static IDS: AtomicUsize = AtomicUsize::new(0);
        let id = IDS.fetch_add(1, Ordering::Acquire);
        let class = unsafe {
            let superclass = class!(NSApplication);
            let mut decl = ClassDecl::new(format!("App{}", id).as_str(), superclass).unwrap();

            decl.register()
        };
        Class(class)
    }
}
