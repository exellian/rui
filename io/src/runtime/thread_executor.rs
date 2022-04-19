use std::cell::{RefCell, UnsafeCell};
use std::sync::Arc;
use std::sync::atomic::{AtomicBool, Ordering};
use std::thread;
use std::thread::JoinHandle;
use crate::alloc::{mpmc, spmc};
use crate::event::Event;
use crate::platform;
use crate::runtime::flow::Flow;
use crate::runtime::task::{DynTask, Status};

pub struct ThreadExecutor {
    poll_flow: AtomicBool,
    gtr_receiver: mpmc::Receiver<RefCell<DynTask>>,
    gtr_sender: mpmc::Sender<RefCell<DynTask>>,
    ltr_sender: spmc::Sender<RefCell<DynTask>>,
    ltr_receiver: spmc::Receiver<RefCell<DynTask>>,
    receiver: spmc::Receiver<Event>,
    // Sender is only accessed through one thread
    // therefore we can use interior mutability
    sender: UnsafeCell<spmc::Sender<Event>>,
    io: UnsafeCell<platform::io::ThreadQueue>,
}

unsafe impl Send for ThreadExecutor {}
unsafe impl Sync for ThreadExecutor {}

impl ThreadExecutor {

    pub fn new(gtr_sender: mpmc::Sender<RefCell<DynTask>>, gtr_receiver: mpmc::Receiver<RefCell<DynTask>>, local_event_queue_size: usize, local_queue_size: usize, flow: Flow) -> Self {
        let (sender, receiver) = spmc::channel(local_queue_size);
        let (ltr_sender, ltr_receiver) = spmc::channel(local_event_queue_size);
        ThreadExecutor {
            poll_flow: AtomicBool::new(flow.into()),
            gtr_receiver,
            gtr_sender,
            ltr_sender,
            ltr_receiver,
            sender: UnsafeCell::new(sender),
            receiver,
            io: UnsafeCell::new(platform::io::ThreadQueue::default()),
        }
    }

    pub fn set_flow(&self, flow: Flow) {
        self.poll_flow.store(flow.into(), Ordering::Release);
    }

    pub fn start(self: Arc<Self>) -> JoinHandle<()> {
        thread::spawn(move || {
            let tq = unsafe { self.io.get().as_mut().unwrap() };
            let sender = unsafe { self.sender.get().as_mut().unwrap() };
            loop {

                match self.ltr_receiver.try_recv() {
                    Some(task) => match task.borrow_mut().poll() {
                        Status::Pending => self.ltr_sender.send(task),
                        Status::Ready => {}
                    },
                    None => {
                        match self.gtr_receiver.try_recv() {
                            None => {}
                            Some(task) => match task.borrow_mut().poll() {
                                Status::Pending => self.
                            }
                        }
                    }
                }


                if self.poll_flow.load(Ordering::Acquire) {
                    tq.poll(sender);
                } else {
                    tq.wait(sender);
                }
            }
        })
    }
}

