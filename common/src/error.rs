use std::{fmt::Display, io, num::ParseIntError};

#[derive(Debug)]
pub enum Error {
    GenericError(String),
    SyntaxError(String),
}

impl Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "Error: {}", self)
    }
}

impl From<io::Error> for Error {
    fn from(v: io::Error) -> Self {
        Error::GenericError(v.to_string())
    }
}

impl From<ParseIntError> for Error {
    fn from(v: ParseIntError) -> Self {
        Error::GenericError(v.to_string())
    }
}
