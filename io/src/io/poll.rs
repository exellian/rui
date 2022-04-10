pub trait Poll {
    type Output;

    fn poll(&self)
}