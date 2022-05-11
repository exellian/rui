use crate::event::Event;
use crate::platform::platform::surface::window::WindowData;
use std::cell::RefCell;
use std::rc::Rc;

pub struct Init {
    pub(crate) callback: Rc<RefCell<dyn FnMut(&Event)>>,
}
impl Init {
    pub fn new(callback: Rc<RefCell<dyn FnMut(&Event)>>) -> Self {
        Init { callback }
    }
}
