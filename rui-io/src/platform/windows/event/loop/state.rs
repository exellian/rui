use crate::event::Event;
use std::cell::RefCell;
use std::mem;
use std::rc::Rc;

pub struct State {
    pub(crate) callback: Rc<RefCell<dyn FnMut(&Event)>>,
}
impl State {
    pub fn new(callback: impl FnMut(&Event)) -> Self {
        let callback = unsafe {
            mem::transmute::<Rc<RefCell<dyn FnMut(&Event)>>, Rc<RefCell<dyn FnMut(&Event)>>>(
                Rc::new(RefCell::new(callback)),
            )
        };
        State { callback }
    }
    
    pub fn call(&mut self, event: &Event) {
        (self.callback.as_ref().borrow_mut())(event)
    } 
}
