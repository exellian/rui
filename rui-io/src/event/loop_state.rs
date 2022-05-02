use std::sync::Arc;
use std::sync::atomic::{AtomicUsize, Ordering};

/// LoopStateRef holds the current state of a loop and is used in conjunction with
/// [`crate::event::loop_control::LoopControl`] to start and stop a loop.
#[derive(Clone)]
pub struct LoopStateRef {
    inner: Arc<AtomicUsize>
}

impl LoopStateRef {
    pub const INIT: usize = 0;
    pub const RUNNING: usize = 1;
    pub const EXITED: usize = 2;

    /// Initializes a [LoopStateRef]
    pub fn new() -> Self {
        LoopStateRef {
            inner: Arc::new(AtomicUsize::new(Self::INIT))
        }
    }

    /// Determines whether the loop is running or not
    pub fn is_running(&self) -> bool {
        self.inner.load(Ordering::Acquire) == Self::RUNNING
    }

    /// Starts the loop
    pub fn start_weak(&self) {
        loop {
            match self.inner.compare_exchange_weak(Self::INIT, Self::RUNNING, Ordering::Release, Ordering::Relaxed) {
                Ok(_) => break,
                Err(x) => match x {
                    Self::INIT => {},
                    _ => break,
                }
            }
        }
    }

    /// Signals the Loop to exit
    pub fn exit(&self) {
        self.inner.store(Self::EXITED, Ordering::Release);
    }
}
