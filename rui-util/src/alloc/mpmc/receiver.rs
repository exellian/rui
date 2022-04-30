use std::sync::Arc;

#[derive(Clone)]
pub struct Receiver<T>(Arc<crossbeam::queue::SegQueue<T>>);

impl<T> Receiver<T> {

    pub fn new(queue: Arc<crossbeam::queue::SegQueue<T>>) -> Self {
        Receiver(queue)
    }

    pub fn try_recv(&self) -> Option<T> {
        self.0.pop()
    }
}