use std::future::Future;
use std::pin::Pin;
use std::sync::Arc;
use std::sync::atomic::{AtomicBool, Ordering};
use std::task::{Context, Poll};
use crate::scheduler::task::output::Output;
use crate::scheduler::task::Status;


pub struct Task<F> where F: Future {
    future: Pin<Box<F>>,
    output: Arc<Output<F::Output>>
}

impl<F> Task<F> where F: Future {

    pub fn new(future: F) -> Self {
        Task {
            future: Box::pin(future),
            output: Arc::new(Output::new())
        }
    }
    
    pub fn output(&self) -> Arc<Output<F::Output>> {
        self.output.clone()
    }

    /// must be only executed from one thread
    pub fn poll(&mut self, cx: &mut Context<'_>) -> Status {
        match self.future.as_mut().poll(cx) {
            // This is safe because the method gets only called from here
            Poll::Ready(out) => unsafe { self.output.put_unchecked(out) },
            Poll::Pending => {}
        }
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