use crate::event::{Event, Flow, InnerLoop};

pub struct MainLoop {}

impl MainLoop {
    pub fn new() -> Self {
        MainLoop {}
    }
}

impl InnerLoop for MainLoop {
    fn wake_up(&self) {
        todo!()
    }

    fn process(&mut self, _: &Flow) -> &mut Self::Queue {
        todo!()
    }
}
