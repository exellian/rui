pub trait Poll {
    type Output;

    fn poll(&self) -> Option<Self::Output>;
}