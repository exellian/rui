use crate::event::Event;
use std::cell::RefCell;
use std::rc::Rc;

pub struct Data {
    callback: Rc<RefCell<dyn FnMut(&Event)>>,
}
impl Data {
    pub fn new(callback: Rc<RefCell<dyn FnMut(&Event)>>) -> Self {
        Data { callback }
    }
    
    pub fn call(&self, event: &Event) {
        (self.callback.as_ref().borrow_mut())(event)
    }
}
