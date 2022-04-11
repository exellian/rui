use std::collections::HashMap;
use std::{mem, thread};
use std::process::exit;
use std::thread::{Thread, ThreadId};
use windows_sys::Win32::Foundation::{BOOL, S_FALSE};
use windows_sys::Win32::UI::WindowsAndMessaging::{GetMessageW, MSG};
use crate::event::Event;
use crate::os_error::OsError;

pub struct Input {
    senders: HashMap<ThreadId, channel::Receiver<Event>>
}

impl Input {

    fn new() -> Self {
        Input {
            senders: HashMap::new()
        }
    }

    pub fn wait(&self) -> Result<Event, OsError> {
        todo!()
    }

    pub fn poll(&self) -> Result<Option<Event>, OsError> {
        let current_thread = thread::current();
        let sender = self.senders.get(&current_thread.id());
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



impl Default for Input {
    fn default() -> Self {
        Input::new()
    }
}