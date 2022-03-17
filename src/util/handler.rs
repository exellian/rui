pub trait Handler<T> {
    type Error;

    fn handle(&mut self, event: T) -> Result<(), Self::Error> {
        Ok(())
    }
}