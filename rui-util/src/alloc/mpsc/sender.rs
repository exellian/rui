use crossbeam::channel::TrySendError;

#[derive(Clone)]
pub struct Sender<T>(crossbeam::channel::Sender<T>);

impl<T> Sender<T> {

    pub fn new(sender: crossbeam::channel::Sender<T>) -> Self {
        Sender(sender)
    }

    pub fn try_send(&self, x: T) -> Result<(), T> {
        match self.0.try_send(x) {
            Ok(_) => Ok(()),
            Err(err) => match err {
                TrySendError::Full(x) => Err(x),
                TrySendError::Disconnected(x) => Err(x)
            }
        }
    }
}