pub struct UnsafeSend<T>(T);
unsafe impl<T> Send for UnsafeSend<T> {}
impl<T> UnsafeSend<T> {
    pub fn new(x: T) -> Self {
        UnsafeSend(x)
    }

    pub fn take(self) -> T {
        self.0
    }
}
