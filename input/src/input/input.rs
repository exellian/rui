use std::ops::Deref;
use std::sync::Arc;
use crate::event::Event;
use crate::input::input_target::InputTarget;
use crate::os_error::OsError;

pub struct Input {
    core: Arc<InputCore>
}

impl !Send for Input {}
impl !Sync for Input {}

impl Input {

    pub fn new() -> Self {
        Input {
            core: Arc::new(InputTarget::new())
        }
    }

    pub fn poll(&self) -> Result<Event, OsError> {
        self.core.poll()
    }

    pub fn wait(&self) -> Result<Event, OsError> {
        self.core.wait()
    }
}

impl Deref for Input {
    type Target = InputTarget;

    fn deref(&self) -> &Self::Target {
        self.core.as_ref()
    }
}