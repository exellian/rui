use std::{fmt, io};
use std::fmt::Formatter;
use std::io::Error;

pub enum OsError {
    IO(io::Error)
}

impl fmt::Debug for OsError {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            OsError::IO(err) => fmt::Debug::fmt(err, f)
        }
    }
}

impl From<io::Error> for OsError {
    fn from(err: Error) -> Self {
        OsError::IO(err)
    }
}