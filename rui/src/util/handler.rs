use std::error::Error;
use std::fmt::Debug;

pub trait Handler<T>
where
    T: Debug,
{
    type Error: Error;

    #[allow(unused_variables)]
    fn handle(&mut self, event: T) -> Result<(), Self::Error> {
        Ok(())
    }
}
