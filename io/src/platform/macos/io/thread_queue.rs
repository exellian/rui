use crate::io::{Poll, Wait};

pub struct ThreadQueue {

}

impl ThreadQueue {
    pub fn new() -> Self {
        ThreadQueue {}
    }
}

impl Poll for ThreadQueue {
    type Output = ();

    fn poll(&self) -> Option<Self::Output> {
        todo!()
    }
}

impl Wait for ThreadQueue {
    type Output = ();

    fn wait(&self) -> Self::Output {
        todo!()
    }
}