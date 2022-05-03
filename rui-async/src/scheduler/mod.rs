mod inner_worker;
pub(crate) mod task;
mod worker;

pub use worker::Worker;

use crate::scheduler::inner_worker::InnerWorker;
use crate::scheduler::task::RawTask;
use rui_util::alloc::mpmc;
use std::collections::HashMap;
use std::ptr::NonNull;
use std::sync::RwLock;

pub struct Scheduler {
    global_sender: mpmc::Sender<RawTask>,
    global_receiver: mpmc::Receiver<RawTask>,
    workers: RwLock<HashMap<usize, NonNull<()>>>,
}
impl Scheduler {
    const DEFAULT_LOCAL_QUEUE_SIZE: usize = 1024;

    pub fn new() -> Self {
        let (global_sender, global_receiver) = mpmc::unbounded();
        Scheduler {
            global_sender,
            global_receiver,
            workers: RwLock::new(HashMap::new()),
        }
    }

    pub fn new_worker(&self) -> Worker {
        Worker::new(Self::DEFAULT_LOCAL_QUEUE_SIZE, self)
    }

    pub(crate) unsafe fn register(&self, worker: &Worker) {
        let mut guard = self.workers.write().unwrap();
        guard.insert(worker.id(), worker.inner_any());
    }

    pub(crate) fn unregister(&self, worker: &Worker) {
        let mut guard = self.workers.write().unwrap();
        guard.remove(&worker.id());
    }

    unsafe fn cast_worker<'this, 'b>(worker: NonNull<()>) -> &'b InnerWorker<'this> {
        worker.cast().as_ref()
    }

    pub fn try_steal<'this>(&'this self, to_id: &usize) -> Option<RawTask> {
        let guard = self.workers.read().unwrap();
        for (id, worker_raw) in guard.iter() {
            if id == to_id {
                continue;
            }
            let worker = unsafe { Self::cast_worker::<'this, '_>(worker_raw.clone()) };
            match worker.try_steal() {
                None => {}
                Some(task) => return Some(task),
            }
        }
        None
    }
}
