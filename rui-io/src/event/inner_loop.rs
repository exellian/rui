use crate::event::{Event, Flow};

/// The Pin guarantees that self does not move. This ensures that it
/// it can safely use self references
pub trait InnerLoop {
    fn wake_up(&self);
    fn init(&mut self, callback: impl FnMut(&Event));
    fn process(&mut self, flow: &Flow);
}
