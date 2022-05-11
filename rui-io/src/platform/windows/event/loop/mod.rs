mod child;
mod main;
mod state;

use crate::event::inner::InnerFlow;
use crate::event::{Event, Flow};
pub use child::Child as ChildLoop;
pub use main::Main as MainLoop;
use state::State as LoopState;
use std::mem;
use windows_sys::Win32::UI::WindowsAndMessaging::{
    DispatchMessageW, GetMessageW, PeekMessageW, TranslateMessage, PM_REMOVE,
};

pub struct Loop {
    state: Option<LoopState>,
}

impl Loop {
    pub fn new() -> Self {
        Loop { state: None }
    }

    pub fn state_mut(&mut self) -> &mut LoopState {
        self.state.as_mut().unwrap()
    }
}
impl crate::event::inner::InnerLoop for Loop {
    fn wake_up(&self) {
        todo!()
    }

    fn init(&mut self, callback: impl FnMut(&Event)) {
        self.state = Some(LoopState::new(callback));
        self.state_mut().call(&Event::Init);
    }

    fn process(&mut self, flow: &InnerFlow) {
        unsafe {
            let mut msg = mem::zeroed();

            let message = match flow {
                InnerFlow::Wait => {
                    GetMessageW(&mut msg, 0, 0, 0);
                    true
                }
                InnerFlow::Poll => PeekMessageW(&mut msg, 0, 0, 0, PM_REMOVE) == true.into(),
            };
            if message {
                TranslateMessage(&msg);
                DispatchMessageW(&msg);
            }
        }
    }
}
