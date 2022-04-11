use std::thread;
use std::sync::atomic::AtomicBool;
use crate::platform;
use crate::platform::io::ThreadQueue;
use crate::runtime::flow::Flow;

pub struct ThreadExecutor {
    poll: AtomicBool,
    thread_queue: platform::io::ThreadQueue,

}
impl ThreadExecutor {

    pub fn new(flow: Flow) -> Self {
        ThreadExecutor {
            poll: AtomicBool::new(flow.into()),
            thread_queue: ThreadQueue::new()
        }
    }

    pub fn start(&self) {

    }
}

