use crate::event::{Event, Flow};

pub trait InnerLoop {

    fn wake_up(&self);
    fn process(&self, flow: &Flow, callback: impl FnMut(Option<&Event>));
}
