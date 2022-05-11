use crate::event::exit_code::ExitCode;
use crate::event::inner::InnerFlow;

/// Flow enumerates different flow control mechanisms for our worker threads.
#[derive(Clone)]
pub enum Flow {
    /// Wait will instruct the thread to wait / block on the given task.
    Wait,
    /// Poll will instruct the thread to repeatedly poll the status of the task.
    Poll,
    /// Exit will instruct the thread to shut itself down.
    Exit(ExitCode),
}
impl TryInto<InnerFlow> for Flow {
    type Error = ();

    fn try_into(self) -> Result<InnerFlow, Self::Error> {
        match self {
            Flow::Wait => Ok(InnerFlow::Wait),
            Flow::Poll => Ok(InnerFlow::Poll),
            Flow::Exit(_) => Err(()),
        }
    }
}
