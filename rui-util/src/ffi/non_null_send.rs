use std::ptr::NonNull;

pub struct NonNullSend<T>(NonNull<T>);
unsafe impl<T> Send for NonNullSend<T> {}

impl<T> NonNullSend<T> {
    pub unsafe fn as_ref<'a>(&self) -> &'a T {
        self.0.as_ref()
    }
}

impl<T> From<&T> for NonNullSend<T> {
    fn from(x: &T) -> Self {
        NonNullSend(NonNull::from(x))
    }
}
