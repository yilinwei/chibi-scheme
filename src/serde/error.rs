use lib_serde::{de, ser};
use std;
use std::fmt::{self, Display};
use std::num::TryFromIntError;

pub type Result<T> = std::result::Result<T, Error>;

#[derive(Clone, Debug, PartialEq)]
pub enum Error {
    Message(String),
    ExpectedBoolean(String),
    ExpectedInteger,
    IntegerTooLargeForBytes(TryFromIntError),
    ExpectedRational,
    ExpectedSymbol,
    ExpectedChar,
    ExpectedPair,
    ExpectedEndOfAssocList,
}

impl ser::Error for Error {
    fn custom<T: Display>(msg: T) -> Self {
        Error::Message(msg.to_string())
    }
}

impl de::Error for Error {
    fn custom<T: Display>(msg: T) -> Self {
        Error::Message(msg.to_string())
    }
}

impl Display for Error {
    fn fmt(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        formatter.write_str(std::error::Error::description(self))
    }
}

impl std::error::Error for Error {
    fn description(&self) -> &str {
        match *self {
            Error::Message(ref msg) => msg,
            _ => unimplemented!(),
        }
    }

    fn cause(&self) -> Option<&std::error::Error> {
        match self {
            Error::IntegerTooLargeForBytes(cause) => Some(cause),
            _ => None,
        }
    }
}
