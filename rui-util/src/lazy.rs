use std::sync::atomic::{AtomicUsize, Ordering};

pub struct Lazy<T> {
    inner: Option<T>,
    state: AtomicUsize,
}
impl<T> Lazy<T> {
    const NONE: usize = 0;
    const INIT: usize = 1;
    const READY: usize = 2;

    pub const fn new() -> Self {
        Lazy {
            inner: None,
            state: AtomicUsize::new(Self::NONE),
        }
    }

    pub fn init(&self, x: T) {
        while let Err(old) = self.state.compare_exchange_weak(
            Self::NONE,
            Self::INIT,
            Ordering::Acquire,
            Ordering::Relaxed,
        ) {
            if old != Self::NONE {
                panic!("Lazy already init!");
            }
        }
        let mut_ref = unsafe { &mut *(&self.inner as *const _ as *mut _) };
        *mut_ref = Some(x);
        self.state.store(Self::READY, Ordering::Release)
    }

    pub fn get(&self) -> Option<&T> {
        match self.state.load(Ordering::Acquire) {
            Self::READY => self.inner.as_ref(),
            _ => None,
        }
    }

    pub fn unwrap(&self) -> &T {
        self.get().unwrap()
    }
}
