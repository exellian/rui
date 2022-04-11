use std::future::Future;
use std::sync::Arc;
use crate::runtime::thread_executor::ThreadExecutor;

pub struct Runtime {
    thread_executors: Vec<Arc<ThreadExecutor>>
}
impl Runtime {
    
    

    fn spawn(&self, task: impl Future) {
        
    }
}