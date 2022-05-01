use std::future::Future;
use std::marker::PhantomData;
use std::sync::atomic::{AtomicUsize, Ordering};
use std::task::{Context, Waker};
use rui_util::alloc::spmc;
use crate::schedular::Scheduler;
use crate::schedular::task::{JoinHandle, RawTask, Status, Task};

pub struct InnerWorker<'scheduler> {
    id: usize,
    local_sender: spmc::Sender<RawTask>,
    local_receiver: spmc::Receiver<RawTask>,
    pub(crate) scheduler: &'scheduler Scheduler,
    // Make the worker non send to ensure thread safety
    _non_send: PhantomData<*const ()>
}
impl<'scheduler> InnerWorker<'scheduler> {

    pub(super) fn new(local_queue_size: usize, scheduler: &'scheduler Scheduler) -> Self {
        static IDS: AtomicUsize = AtomicUsize::new(0);
        let (local_sender, local_receiver) = spmc::channel(local_queue_size);
        InnerWorker {
            id: IDS.fetch_add(1, Ordering::Acquire),
            local_sender,
            local_receiver,
            scheduler,
            _non_send: Default::default()
        }
    }

    pub fn id(&self) -> usize {
        self.id
    }

    pub fn spawn<F>(&self, task: F) -> JoinHandle<F::Output> where
        F: Future + 'scheduler
    {
        let task= Task::new(task);
        let handle = JoinHandle::new(task.output());
        // This is safe because the InnerWorker cannot outlive 'scheduler and so the tasks either
        let raw_task = unsafe { RawTask::new_unchecked(task) };
        self.scheduler.global_sender.send(raw_task);
        handle
    }

    fn next_task(&self) -> Option<RawTask> {
        match self.scheduler.global_receiver.try_recv() {
            None => match self.local_receiver.try_recv() {
                None => self.scheduler.try_steal(&self.id),
                Some(task) => Some(task)
            },
            Some(task) => Some(task)
        }
    }

    fn queue_task(&mut self, task: RawTask) {
        match self.local_sender.try_send(task) {
            Ok(_) => {}
            Err(task) => self.scheduler.global_sender.send(task)
        }
    }

    pub fn poll(&mut self) {
        match self.next_task() {
            None => {}
            Some(mut task) => match unsafe { task.poll() } {
                Status::Pending => self.queue_task(task),
                Status::Ready => {}
            }
        }
    }

    pub(crate) fn try_steal(&self) -> Option<RawTask> {
        self.local_receiver.try_recv()
    }
}