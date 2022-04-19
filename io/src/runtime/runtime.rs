use std::future::Future;
use std::sync::Arc;
use crate::runtime::task::JoinHandle;
use crate::runtime::thread_executor::ThreadExecutor;

pub struct Runtime {
    thread_executors: Vec<Arc<ThreadExecutor>>
}

impl Runtime {

    fn new() -> Self {
        Runtime {
            thread_executors: vec![]
        }
    }

    fn spawn<F>(&self, task: F) -> JoinHandle<F::Output> where F: Future {

        JoinHandle::new()
    }
}