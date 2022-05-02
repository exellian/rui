use crate::event::exit_code::ExitCode;

/// Flow enumerates different flow control mechanisms for our worker threads.
#[derive(Clone)]
pub enum Flow {
    /// Wait will instruct the thread to wait / block on the given task.
    Wait,
    /// Poll will instruct the thread to repeatedly poll the status of the task.
    Poll,
    /// Exit will instruct the thread to shut itself down.
    Exit(ExitCode)
}