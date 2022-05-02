use std::cell::UnsafeCell;
use std::mem::MaybeUninit;
use std::sync::atomic::{AtomicUsize, Ordering};
use std::task::Poll;

pub struct Output<T> {
    value: UnsafeCell<MaybeUninit<T>>,
    state: AtomicUsize
}

impl<T> Output<T> {

    const Pending: usize = 0;
    const Ready: usize = 1;
    const Consumed: usize = 2;

    pub fn new() -> Self {
        Output {
            value: UnsafeCell::new(MaybeUninit::uninit()),
            state: AtomicUsize::new(Self::Pending)
        }
    }

    pub unsafe fn put_unchecked(&self, x: T) {
        self.value.get().write(MaybeUninit::new(x));
        self.state.store(Self::Ready, Ordering::Release);
    }

    pub fn poll_consume(&self) -> Poll<T> {
        let state = self.state.load(Ordering::Acquire);
        match state {
            Self::Pending => Poll::Pending,
            Self::Ready => {
                while let Err(val) = self.state.compare_exchange_weak(state, Self::Consumed, Ordering::Release, Ordering::Relaxed) {
                    if val == Self::Consumed {
                        panic!("Output value already consumed!");
                    }
                }
                let mut out = MaybeUninit::uninit();
                unsafe { self.value.get().swap(&mut out) };
                Poll::Ready(unsafe { out.assume_init() })
            },
            Self::Consumed => panic!("Output value already consumed!"),
            _ => unreachable!()
        }
    }
}