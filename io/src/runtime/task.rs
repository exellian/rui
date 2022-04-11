use std::future::Future;

pub struct Task {

}

impl Task {

    fn new<F>(future: F) -> Self where F: Future {
        Task {

        }
    }
}