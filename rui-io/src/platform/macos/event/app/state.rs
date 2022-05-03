use crate::platform::event::Queue;

pub struct State {
    queue: Queue,
}
impl State {
    pub fn new(queue: Queue) -> Self {
        State { queue }
    }
}
