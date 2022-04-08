use crate::event::Event;
use crate::os_error::OsError;

pub struct InputTarget {

}
impl InputTarget {

    pub fn new() -> Self {
        InputCore {}
    }

    fn poll(&self) -> Result<Event, OsError> {
        todo!()
    }

    fn wait(&self) -> Result<Event, OsError> {
        todo!()
    }
}