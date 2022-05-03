use crate::event::Event;
use std::borrow::BorrowMut;
use std::cell::RefCell;
use std::collections::VecDeque;
use std::rc::Rc;

#[derive(Clone)]
pub struct Queue {
    inner: Rc<RefCell<VecDeque<Event>>>,
}
impl Queue {
    pub fn new() -> Self {
        Queue {
            inner: Rc::new(RefCell::new(VecDeque::new())),
        }
    }
}

impl crate::event::queue::Enqueue<Event> for Queue {
    fn enqueue(&mut self, x: Event) {
        self.inner.as_ref().borrow_mut().push_back(x);
    }
}

impl crate::event::queue::Dequeue<Event> for Queue {
    fn dequeue(&mut self) -> Option<Event> {
        self.inner.as_ref().borrow_mut().pop_front()
    }
}

impl crate::event::Queue<Event> for Queue {}
