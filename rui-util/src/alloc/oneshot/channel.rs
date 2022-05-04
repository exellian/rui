use std::mem;
use std::sync::atomic::{AtomicBool, Ordering};

pub struct Channel<T> {
    init: AtomicBool,
    val: Option<T>,
}
impl<T> Channel<T> {
    pub(crate) fn new() -> Self {
        Channel {
            init: AtomicBool::new(false),
            val: None,
        }
    }

    /// It must be ensured that this method never gets called after the value was taken
    pub(crate) unsafe fn try_recv(&self) -> Option<T> {
        return if self.init.load(Ordering::Acquire) {
            let mut_this = &mut *(self as *const Self as *mut Self);
            mut_this.val.take()
        } else {
            None
        };
    }

    /// It must be ensured that this method only gets called one time
    pub(crate) unsafe fn send(&self, x: T) {
        let mut_this = &mut *(self as *const Self as *mut Self);
        let mut val = Some(x);
        mem::swap(&mut mut_this.val, &mut val);
        self.init.store(true, Ordering::Release);
    }
}
