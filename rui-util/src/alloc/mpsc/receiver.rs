pub struct Receiver<T>(crossbeam::channel::Receiver<T>);

impl<T> Receiver<T> {

    pub fn new(recv: crossbeam::channel::Receiver<T>) -> Self {
        Receiver(recv)
    }

    pub fn try_recv(&mut self) -> Option<T> {
        match self.0.try_recv() {
            Ok(res) => Some(res),
            Err(_) => None
        }
    }
}