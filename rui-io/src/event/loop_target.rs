use std::mem;

use crate::event::child_loop::ChildLoop;
use rui_util::ffi::NonNullSend;

use crate::event::exit_code::ExitCode;
use crate::event::loop_control::LoopControl;
use crate::event::loop_state::LoopStateRef;
use crate::event::main_loop::MainLoop;

#[derive(Clone)]
pub enum LoopTarget<'main, 'child> {
    Main(&'main MainLoop),
    Child(&'child ChildLoop<'main>),
}
impl<'main, 'child> LoopTarget<'main, 'child>
where
    'child: 'main,
{
    /// Because of the special loop structure (theres always a main loop that outlives every child loop)
    /// the callback doesn't need to be static lifetime but only has to fulfill 'main lifetime
    pub fn spawn<F>(&self, callback: F)
    where
        F: for<'new_child> FnOnce(&'new_child ChildLoop<'main>) -> ExitCode + 'main + Send,
    {
        // The main loop lives for 'main
        let main: &'main MainLoop = match self {
            LoopTarget::Main(m) => *m,
            LoopTarget::Child(child_loop) => child_loop.main,
        };

        let callback_box = unsafe {
            mem::transmute::<
                Box<
                    dyn for<'new_child> FnOnce(&'new_child ChildLoop<'main>) -> ExitCode
                        + 'main
                        + Send,
                >,
                Box<
                    dyn for<'new_child> FnOnce(&'new_child ChildLoop<'static>) -> ExitCode
                        + 'static
                        + Send,
                >,
            >(Box::new(callback))
        };

        let main_ptr = NonNullSend::from(main);
        let loop_state = LoopStateRef::new();
        let loop_state_ret = loop_state.clone();
        {
            let mut controls_guard = main.child_loop_controls.write().unwrap();
            let thread_handle = unsafe {
                // TODO Use experimental feature to
                std::thread::spawn(move || {
                    // This is only safe because we ae making sure that the main loop was created before
                    // any other child thread and that the main loop lives the longest
                    let main = main_ptr.as_ref();
                    let local_loop = ChildLoop::new(main, loop_state_ret);

                    // We cant safely call this because all child threads get joined when the main loop ends
                    callback_box(&local_loop)
                })
            };
            controls_guard.push(LoopControl::new(loop_state, thread_handle))
        }
    }
}
