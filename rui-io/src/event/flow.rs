use crate::event::exit_code::ExitCode;

/// Flow enumerates different flow control mechanisms for use in event loops.
#[derive(Clone)]
pub enum Flow {
    /// Wait will instruct the event loop to wait / block on the given task.
    Wait,
    /// Poll will instruct the event loop to repeatedly poll the status of the task.
    Poll,
    /// Exit will instruct the loop to shut itself down.
    Exit(ExitCode)
}