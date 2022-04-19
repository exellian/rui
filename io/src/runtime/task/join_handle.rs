use std::future::Future;
use std::marker::PhantomData;
use std::pin::Pin;
use std::sync::Arc;
use std::task::{Context, Poll};
use crate::runtime::task::dyn_task::DynTask;

pub struct JoinHandle<T> {
    task: Arc<DynTask>,
    _t: PhantomData<T>
}
impl<T> JoinHandle<T> {

    pub fn new(task: Arc<DynTask>) -> Self {
        JoinHandle {
            task,
            _t: PhantomData
        }
    }
}
impl<T> Future for JoinHandle<T> where T: 'static {
    type Output = T;

    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        self.task.poll_consume::<T>()
    }
}