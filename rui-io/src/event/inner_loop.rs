use crate::event::{Event, Flow, Queue};

/// The Pin guarantees that self does not move. This ensures that it
/// it can safely use self references
pub trait InnerLoop {
    type Queue: Queue<Event>;

    fn wake_up(&self);
    fn process(&mut self, flow: &Flow) -> &mut Self::Queue;
}
