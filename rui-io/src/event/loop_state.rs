use std::sync::Arc;
use std::sync::atomic::{AtomicUsize, Ordering};

#[derive(Clone)]
pub struct LoopStateRef {
    inner: Arc<AtomicUsize>
}
impl LoopStateRef {
    pub const Init: usize = 0;
    pub const Running: usize = 1;
    pub const Exited: usize = 2;

    pub fn new() -> Self {
        LoopStateRef {
            inner: Arc::new(AtomicUsize::new(Self::Init))
        }
    }

    pub fn is_running(&self) -> bool {
        self.inner.load(Ordering::Acquire) == Self::Running
    }

    pub fn start_weak(&self) {
        loop {
            match self.inner.compare_exchange_weak(Self::Init, Self::Running, Ordering::Release, Ordering::Relaxed) {
                Ok(_) => break,
                Err(x) => match x {
                    Self::Init => {},
                    _ => break,
                }
            }
        }
    }

    pub fn exit(&self) {
        self.inner.store(Self::Exited, Ordering::Release);
    }
}