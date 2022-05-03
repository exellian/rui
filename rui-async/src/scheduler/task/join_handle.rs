use crate::scheduler::task::output::Output;
use std::future::Future;
use std::marker::PhantomData;
use std::pin::Pin;
use std::sync::Arc;
use std::task::{Context, Poll};

pub struct JoinHandle<T> {
    output: Arc<Output<T>>,
    _t: PhantomData<T>,
}
impl<T> JoinHandle<T> {
    pub fn new(output: Arc<Output<T>>) -> Self {
        JoinHandle {
            output,
            _t: PhantomData,
        }
    }
}
impl<T> Future for JoinHandle<T>
where
    T: 'static,
{
    type Output = T;

    fn poll(self: Pin<&mut Self>, _: &mut Context<'_>) -> Poll<Self::Output> {
        self.output.poll_consume()
    }
}
