use std::future::Future;
use std::ptr::NonNull;
use crate::schedular::inner_worker::InnerWorker;
use crate::schedular::Scheduler;
use crate::schedular::task::{JoinHandle, RawTask};

pub struct Worker<'scheduler> {
    inner: Box<InnerWorker<'scheduler>>
}
impl<'scheduler> Worker<'scheduler> {

    pub fn new(local_queue_size: usize, scheduler: &'scheduler Scheduler) -> Self {
        let worker = Worker {
            inner: Box::new(InnerWorker::new(local_queue_size, scheduler))
        };
        unsafe { scheduler.register(&worker)}
        worker
    }

    pub fn id(&self) -> usize {
        self.inner.id()
    }

    pub fn spawn<F>(&self, task: F) -> JoinHandle<F::Output> where F: Future + 'scheduler {
        self.inner.spawn(task)
    }

    pub fn poll(&mut self) {
        self.inner.poll()
    }

    pub(crate) fn try_steal(&self) -> Option<RawTask> {
        self.inner.try_steal()
    }

    pub(crate) fn inner_any(&self) -> NonNull<()> {
        unsafe { NonNull::from(self.inner.as_ref()).cast() }
    }
}
impl<'scheduler> Drop for Worker<'scheduler> {
    fn drop(&mut self) {
        self.inner.scheduler.unregister(self)
    }
}