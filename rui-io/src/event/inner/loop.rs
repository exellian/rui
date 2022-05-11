use crate::event::inner::InnerFlow;
use crate::event::Event;

/// The Pin guarantees that self does not move. This ensures that it
/// it can safely use self references
pub trait Loop {
    fn wake_up(&self);
    fn init(&mut self, callback: impl FnMut(&Event));
    fn process(&mut self, flow: &InnerFlow);
}
