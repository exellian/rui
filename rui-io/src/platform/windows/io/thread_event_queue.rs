use crate::alloc::spmc::Sender;
use crate::event::Event;

pub struct ThreadEventQueue {

}

impl ThreadEventQueue {
    fn new() -> Self {
        ThreadEventQueue {}
    }

    pub fn wait(&mut self, sender: &mut Sender<Event>) {

    }

    pub fn poll(&mut self, sender: &mut Sender<Event>) {
        unsafe {
            let mut msg = mem::zeroed();

            match GetMessageW(&mut msg, 0, 0, 0) {
                S_FALSE => {
                    exit(0)
                },
                _ => {
                    Ok(Some((&mut msg as *mut _ as u32).into()))
                }
            }
        }
    }
}

impl Default for ThreadEventQueue {
    fn default() -> Self {
        ThreadEventQueue::new()
    }
}
