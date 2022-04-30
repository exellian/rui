use crate::event::{Event, Flow, InnerLoop};

pub struct Loop {

}
impl Loop {

    pub fn new() -> Self {
        Loop {}
    }
}
impl InnerLoop for Loop {

    fn wake_up(&self) {
        todo!()
    }

    fn process(&self, flow: &Flow, callback: impl FnMut(Option<&Event>)) {
        todo!()
    }
}
