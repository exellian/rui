#[derive(Copy, Clone)]
pub enum Flow {
    Wait,
    Poll
}
impl Into<bool> for Flow {
    fn into(self) -> bool {
        match self {
            Flow::Wait => false,
            Flow::Poll => true
        }
    }
}