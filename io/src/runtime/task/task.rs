use std::cell::UnsafeCell;
use std::future::Future;
use std::mem;
use std::sync::atomic::{AtomicBool, Ordering};
use std::task::Poll;
use crate::runtime::task::Status;


pub enum Output<T> {
    Pending,
    Ready(T),
    Consumed
}

pub struct Task<F> where F: Future {
    future: F,
    output: UnsafeCell<Output<F::Output>>,
    consumed: AtomicBool
}

impl<F> Task<F> where F: Future {

    pub fn new(future: F) -> Self {
        Task {
            future,
            output: UnsafeCell::new(Output::Pending),
            consumed: AtomicBool::new(false)
        }
    }

    pub fn poll_consume(&self) -> Poll<F::Output> {
        if let Err(v) = self.consumed.compare_exchange_weak(false, true, Ordering::Acquire, Ordering::Relaxed) {
            if v {
                panic!("Task already consumed!");
            }
            Poll::Pending
        } else {
            let mut out = Output::Consumed;
            mem::swap(&mut out, unsafe { self.output.get().as_mut().unwrap() });
            match out {
                Output::Ready(o) => Poll::Ready(o),
                Output::Pending => Poll::Pending,
                _ => panic!()
            }

        }
    }

    pub fn poll(&mut self) -> Status {
        Status::Ready
    }

    /*
    unsafe fn future_unchecked<F>(&mut self) -> Pin<&mut F> where F: 'static {
        Pin::new_unchecked(self.future.downcast_mut_unchecked())
    }

    fn future<F>(&mut self) -> Option<Pin<&mut F>> where F: 'static {
        let casted_future = self.future.downcast_mut();
        match casted_future {
            // It is always safe to perform the pinning of a pointer to box
            Some(fut) => unsafe {
                Some(Pin::new_unchecked(fut))
            },
            None => None
        }
    }
    */
}