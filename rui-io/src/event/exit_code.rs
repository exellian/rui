#[derive(Clone)]
pub enum ExitCode {
    Default = 0,
    Error = 1
}
impl ExitCode {

    pub fn is_success(&self) -> bool {
        match self {
            ExitCode::Default => true,
            _ => false
        }
    }
}
impl Into<i32> for ExitCode {
    fn into(self) -> i32 {
        self as i32
    }
}