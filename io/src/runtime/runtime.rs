use std::future::Future;
use crate::runtime::thread_executor::ThreadExecutor;

pub struct Runtime {
    thread_executors: Vec<ThreadExecutor>
}
impl Runtime {

    fn spawn(&self, task: impl Future) {
        
    }
}