use crate::event::queue::Enqueue;
use crate::event::Event;
use crate::platform::event::Queue;
use std::cell::RefCell;
use std::rc::Rc;

pub struct DelegateState {
    callback: Rc<RefCell<dyn FnMut(&Event)>>,
}
impl DelegateState {
    pub fn new(callback: Rc<RefCell<dyn FnMut(&Event)>>) -> Self {
        DelegateState { callback }
    }

    pub fn did_finish_launching(&mut self) {
        (self.callback.borrow_mut())(&Event::Init);
    }
}
