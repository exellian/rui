use std::thread::JoinHandle;
use crate::event::exit_code::ExitCode;
use crate::event::loop_state::LoopStateRef;

pub struct LoopControl {
    loop_state: LoopStateRef,
    thread_handle: JoinHandle<ExitCode>
}
impl LoopControl {
    pub fn new(loop_state: LoopStateRef, thread_handle: JoinHandle<ExitCode>) -> Self {
        LoopControl {
            loop_state,
            thread_handle
        }
    }

    pub fn signal_exit(&self) {
        self.loop_state.exit()
    }

    pub fn join(self) -> ExitCode {
        self.thread_handle.join().unwrap()
    }
}