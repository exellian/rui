use crate::event::event::Event;

impl<'a, T> TryFrom<winit::event::Event<'a, T>> for Event<T> {
    type Error = ();

    fn try_from(value: winit::event::Event<'a, T>) -> Result<Self, Self::Error> {
        todo!()
    }
}