use std::future::Future;

pub struct IO {

}
impl IO {

    pub fn new() -> Self {
        IO {}
    }

    pub fn run(task: impl Future) {
        
    }
}
