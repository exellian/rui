use std::cell::{RefCell, UnsafeCell};
use std::sync::Arc;
use std::sync::atomic::{AtomicBool, Ordering};
use std::task::Poll;
use std::thread;
use std::thread::JoinHandle;
use crate::alloc::{mpmc, spmc};
use crate::event::Event;
use crate::platform;
use crate::runtime::flow::Flow;
use crate::runtime::task::{DynTask, Status};

pub struct ThreadExecutor {
    poll_flow: AtomicBool,
    running: AtomicBool,
    gtr_receiver: mpmc::Receiver<Arc<DynTask>>,
    gtr_sender: mpmc::Sender<Arc<DynTask>>,
    ltr_receiver: spmc::Receiver<Arc<DynTask>>,
    ltr_sender: UnsafeCell<spmc::Sender<Arc<DynTask>>>,
    // Sender is only accessed through one thread
    // therefore we can use interior mutability
    event_sender: UnsafeCell<mpmc::Sender<Event>>,
    io: UnsafeCell<platform::io::ThreadEventQueue>,
}

unsafe impl Send for ThreadExecutor {}
unsafe impl Sync for ThreadExecutor {}

impl ThreadExecutor {

    pub fn new(
        gtr_sender: mpmc::Sender<Arc<DynTask>>,
        gtr_receiver: mpmc::Receiver<Arc<DynTask>>,
        event_sender: mpmc::Sender<Event>,
        local_queue_size: usize,
        flow: Flow
    ) -> Self {
        let (ltr_sender, ltr_receiver) = spmc::channel(local_queue_size);
        ThreadExecutor {
            poll_flow: AtomicBool::new(flow.into()),
            running: AtomicBool::new(false),
            gtr_receiver,
            gtr_sender,
            ltr_receiver,
            ltr_sender: UnsafeCell::new(ltr_sender),
            event_sender: UnsafeCell::new(event_sender),
            io: UnsafeCell::new(platform::io::ThreadEventQueue::default()),
        }
    }

    pub fn set_flow(&self, flow: Flow) {
        self.poll_flow.store(flow.into(), Ordering::Release);
    }

    // Runs the executor until the specified task is ready
    pub fn run_for_task<O>(self: Arc<Self>, task: Arc<DynTask>) -> O where O: 'static {
        let mut result = None;
        Self::run_while(self, || {
            match task.poll_consume() {
                Poll::Ready(res) => {
                    result = Some(res);
                    false
                },
                Poll::Pending => {
                    true
                }
            }
        });
        result.unwrap()
    }

    pub fn run(self: Arc<Self>) {
        self.running.store(true, Ordering::Release);
        self.clone().run_while(|| self.running.load(Ordering::Acquire))
    }

    fn run_while<F>(self: Arc<Self>, mut condition: F) where F: FnMut() -> bool {
        let tq = unsafe { self.io.get().as_mut().unwrap() };
        let event_sender = unsafe { self.event_sender.get().as_mut().unwrap() };
        let ltr_sender = unsafe { self.ltr_sender.get().as_mut().unwrap() };
        while condition() {
            let mut full = false;
            let mut polled = false;
            // Try to take a task from the local queue
            match self.ltr_receiver.try_recv() {
                // Poll the task
                Some(task) => {
                    polled = true;
                    match unsafe { task.poll() } {
                        // Try adding the task to the local queue
                        // so that the task gets polled
                        Status::Pending => match ltr_sender.try_send(task) {
                            Ok(_) => {}
                            // If the local queue is full add the task to the global queue
                            Err(task) => {
                                full = true;
                                self.gtr_sender.send(task)
                            }
                        },
                        // drop the ready polled task
                        Status::Ready => {}
                    }
                },
                // If the local queue is empty take tasks from the global queue
                None => {}
            }
            if !full {
                match self.gtr_receiver.try_recv() {
                    // If the global queue is also empty
                    // do some work stealing from other threads
                    None => {
                        // TODO work stealing from other threads
                    }
                    Some(task) => {
                        polled = true;
                        match unsafe { task.poll() } {
                            Status::Pending => match ltr_sender.try_send(task) {
                                Ok(_) => {}
                                // If the local queue is full add the task to the global queue
                                Err(task) => self.gtr_sender.send(task)
                            },
                            // Drop the ready polled task
                            Status::Ready => {}
                        }
                    }
                }
            }
            // If we polled a task - so we still have non finished tasks - then we should
            // also poll os events and don't wait
            if self.poll_flow.load(Ordering::Acquire) || polled {
                tq.poll(event_sender);
            }
            // If we have an empty task queue then we reduce the cpu load by waiting for the next
            // OS event if the thread executor follows a Flow::wait policy.
            else {
                tq.wait(event_sender);
            }
        }
    }
}

