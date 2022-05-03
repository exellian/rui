use std::sync::Arc;

#[derive(Clone)]
pub struct Sender<T>(Arc<crossbeam::queue::SegQueue<T>>);

impl<T> Sender<T> {
    pub fn new(queue: Arc<crossbeam::queue::SegQueue<T>>) -> Self {
        Sender(queue)
    }

    pub fn send(&self, x: T) {
        self.0.push(x)
    }
}
