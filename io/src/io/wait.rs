pub trait Wait {
    type Output;

    fn wait(&self) -> Self::Output;
}