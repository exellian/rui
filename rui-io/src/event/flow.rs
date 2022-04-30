use crate::event::exit_code::ExitCode;

#[derive(Clone)]
pub enum Flow {
    Wait,
    Poll,
    Exit(ExitCode)
}