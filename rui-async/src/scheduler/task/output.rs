use std::cell::UnsafeCell;
use std::mem::MaybeUninit;
use std::sync::atomic::{AtomicUsize, Ordering};
use std::task::Poll;

pub struct Output<T> {
    value: UnsafeCell<MaybeUninit<T>>,
    state: AtomicUsize,
}

impl<T> Output<T> {
    const PENDING: usize = 0;
    const READY: usize = 1;
    const CONSUMED: usize = 2;

    pub fn new() -> Self {
        Output {
            value: UnsafeCell::new(MaybeUninit::uninit()),
            state: AtomicUsize::new(Self::PENDING),
        }
    }

    pub unsafe fn put_unchecked(&self, x: T) {
        self.value.get().write(MaybeUninit::new(x));
        self.state.store(Self::READY, Ordering::Release);
    }

    pub fn poll_consume(&self) -> Poll<T> {
        let state = self.state.load(Ordering::Acquire);
        match state {
            Self::PENDING => Poll::Pending,
            Self::READY => {
                while let Err(val) = self.state.compare_exchange_weak(
                    state,
                    Self::CONSUMED,
                    Ordering::Release,
                    Ordering::Relaxed,
                ) {
                    if val == Self::CONSUMED {
                        panic!("Output value already consumed!");
                    }
                }
                let mut out = MaybeUninit::uninit();
                unsafe { self.value.get().swap(&mut out) };
                Poll::Ready(unsafe { out.assume_init() })
            }
            Self::CONSUMED => panic!("Output value already consumed!"),
            _ => unreachable!(),
        }
    }
}
