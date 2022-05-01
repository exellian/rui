use std::{mem, thread};
use std::mem::MaybeUninit;
use rui_util::ffi::{NonNullSend, UnsafeSend};
use crate::event::Event;
use crate::event::exit_code::ExitCode;
use crate::event::loop_control::LoopControl;
use crate::event::loop_state::LoopStateRef;
use crate::event::main_loop::MainLoop;
use crate::event::r#loop::Loop;

#[derive(Clone)]
pub enum LoopTarget<'main, 'child> {
    Main(&'main MainLoop),
    Child(&'child Loop<'main>)
}
impl<'main, 'child> LoopTarget<'main, 'child> where 'child: 'main {

    /// Because of the special loop structure (theres always a main loop that outlives every child loop)
    /// the callback doesn't need to be static lifetime but only has to fulfill 'main lifetime
    pub fn spawn<F>(&self, callback: F) where
        F: for<'new_child> FnOnce(&'new_child Loop<'main>) -> ExitCode + 'main + Send
    {
        // The main loop lives for 'main
        let main: &'main MainLoop = match self {
            LoopTarget::Main(m) => *m,
            LoopTarget::Child(child_loop) => child_loop.main
        };
        let main_ptr = NonNullSend::from(main);
        let loop_state = LoopStateRef::new();
        let loop_state_ret = loop_state.clone();
        {
            let mut controls_guard = main.child_loop_controls.write().unwrap();
            let thread_handle = unsafe {
                // Use experimental feature to
                thread::Builder::new().spawn_unchecked(move || {

                    // This is only safe because we ae making sure that the main loop was created before
                    // any other child thread and that the main loop lives the longest
                    let main = unsafe { main_ptr.as_ref() };
                    let local_loop = Loop::new(main, loop_state_ret);
                    callback(&local_loop)
                }).unwrap()
            };
            controls_guard.push(LoopControl::new(loop_state, thread_handle))
        }
    }
}