use std::io;
use std::io::Error;

pub enum OsError {
    IO(io::Error)
}

impl From<io::Error> for OsError {
    fn from(err: Error) -> Self {
        OsError::IO(err)
    }
}