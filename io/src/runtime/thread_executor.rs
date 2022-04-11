use std::cell::{RefCell, UnsafeCell};
use std::sync::Arc;
use std::sync::atomic::{AtomicBool, Ordering};
use std::thread;
use std::thread::JoinHandle;
use crate::alloc::spmc;
use crate::event::Event;
use crate::platform;
use crate::runtime::flow::Flow;

pub struct ThreadExecutor {
    poll_flow: AtomicBool,
    receiver: spmc::Receiver<Event>,
    // Sender is only accessed through one thread
    // therefore we can use interior mutability
    sender: UnsafeCell<spmc::Sender<Event>>,
    io: UnsafeCell<platform::io::IO>
}

unsafe impl Send for ThreadExecutor {}
unsafe impl Sync for ThreadExecutor {}

impl ThreadExecutor {

    pub fn new(local_queue_size: usize, flow: Flow) -> Self {
        let (sender, receiver) = spmc::channel(local_queue_size);
        ThreadExecutor {
            poll_flow: AtomicBool::new(flow.into()),
            sender: UnsafeCell::new(sender),
            receiver,
            io: UnsafeCell::new(platform::io::IO::default())
        }
    }

    pub fn set_flow(&self, flow: Flow) {
        self.poll_flow.store(flow.into(), Ordering::Release);
    }

    pub fn start(self: Arc<Self>) -> JoinHandle<()> {
        thread::spawn(move || {
            let io = self.io.get_mut();
            let sender = self.sender.get_mut();
            loop {
                if self.poll_flow.load(Ordering::Acquire) {
                    io.poll(sender);
                } else {
                    io.wait(sender);
                }
            }
        })
    }
}

