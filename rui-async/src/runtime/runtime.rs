use std::future::Future;
use std::sync::{Arc, Mutex};
use std::sync::atomic::{AtomicUsize, Ordering};
use std::thread;
use rui_util::alloc::mpmc;
use crate::event::Event;
use crate::runtime::event_future::EventFuture;
use crate::runtime::flow::Flow;
use crate::runtime::task::{DynTask, JoinHandle};
use crate::runtime::thread_executor::ThreadExecutor;

pub struct Runtime {
    thread_executors: Mutex<Vec<Arc<ThreadExecutor>>>,
    local_thread_executor: Arc<ThreadExecutor>,
    gtr_sender: mpmc::Sender<Arc<DynTask>>,
    gtr_receiver: mpmc::Receiver<Arc<DynTask>>,
    event_sender: mpmc::Sender<Event>,
    event_receiver: mpmc::Receiver<Event>,
    max_threads: usize,
    thread_create_threshold: usize,
    thread_delete_threshold: usize,
    task_counter: Arc<AtomicUsize>,
    thread_counter: AtomicUsize,
}

impl Runtime {

    // Use different threshold for creating and deleting a thread
    // to reduce costly thread creation times
    const THREAD_CREATE_THRESHOLD: usize = 100;
    const THREAD_DELETE_THRESHOLD: usize = 50;
    const LOCAL_TASK_QUEUE_SIZE: usize = 512;

    pub fn new(max_threads: usize) -> Self {
        let (gtr_sender, gtr_receiver) = mpmc::unbounded();
        let (event_sender, event_receiver) = mpmc::unbounded();
        let local_thread_executor = Arc::new(ThreadExecutor::new(
            gtr_sender.clone(),
            gtr_receiver.clone(),
            event_sender.clone(),
            Self::LOCAL_TASK_QUEUE_SIZE,
            Flow::Wait
        ));
        Runtime {
            thread_executors: Mutex::new(vec![]),
            local_thread_executor,
            gtr_sender,
            gtr_receiver,
            event_sender,
            event_receiver,
            max_threads,
            thread_create_threshold: Self::THREAD_CREATE_THRESHOLD,
            thread_delete_threshold: Self::THREAD_DELETE_THRESHOLD,
            task_counter: Arc::new(AtomicUsize::new(0)),
            thread_counter: AtomicUsize::new(0)
        }
    }

    fn wake_up(&self) {

    }

    pub fn run<F>(&self, task: F) -> F::Output where F: Future + 'static {
        let dyn_task = Arc::new(DynTask::new(task));
        self.local_thread_executor.clone().run_for_task(dyn_task)
    }

    pub fn recv_event(&self) -> EventFuture {
        EventFuture::new(&self.event_receiver)
    }

    pub fn spawn<F>(&self, task: F) -> JoinHandle<F::Output> where F: Future + 'static {
        loop {
            let active = self.task_counter.load(Ordering::Acquire);
            let thread_count = self.thread_counter.load(Ordering::Acquire);
            if thread_count != 0 && (active % thread_count <= self.thread_create_threshold || thread_count == self.max_threads) {
                break;
            }
            {
                let mut guard = self.thread_executors.lock().unwrap();
                match self.thread_counter.compare_exchange_weak(thread_count, thread_count + 1, Ordering::Release, Ordering::Relaxed) {
                    // If the exchange succeeded we can safely create a new thread executor
                    Ok(_) => {
                        let thread_executor = Arc::new(ThreadExecutor::new(
                            self.gtr_sender.clone(),
                            self.gtr_receiver.clone(),
                            self.event_sender.clone(),
                            Self::LOCAL_TASK_QUEUE_SIZE,
                            Flow::Wait
                        ));
                        guard.push(thread_executor.clone());
                        thread::spawn(move || {
                            thread_executor.run();
                        });
                    },
                    // Try again a different thread created a new thread in the meantime
                    Err(_) => {}
                }
            }
        }
        let dyn_task = Arc::new(DynTask::new(task));
        // Simply send the new task to the global queue
        self.gtr_sender.send(dyn_task.clone());
        self.wake_up();
        JoinHandle::new(dyn_task)
    }
}
