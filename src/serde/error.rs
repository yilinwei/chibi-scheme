use lib_serde::{de, ser};
use std;
use std::fmt::{self, Display};
use std::num::TryFromIntError;

pub type Result<T> = std::result::Result<T, Error>;

#[derive(Clone, Debug, PartialEq)]
pub enum Error {
    DeserializeAnyNotSupported,
    DeserializeIgnoredAnyNotSupported,
    Message(String),
    ExpectedBoolean(String),
    ExpectedInteger(String),
    IntegerTooLargeForBytes(TryFromIntError),
    ExpectedRational(String),
    ExpectedSymbol(String),
    ExpectedChar(String),
    ExpectedString(String),
    ExpectedPairOrEndOfAssocList(String),
    ExpectedPair(String),
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
            Error::ExpectedBoolean(ref msg) => msg,
            Error::ExpectedInteger(ref msg) => msg,
            Error::ExpectedRational(ref msg) => msg,
            Error::ExpectedSymbol(ref msg) => msg,
            Error::ExpectedChar(ref msg) => msg,
            Error::ExpectedString(ref msg) => msg,
            Error::ExpectedPairOrEndOfAssocList(ref msg) => msg,
            Error::ExpectedPair(ref msg) => msg,
            Error::DeserializeAnyNotSupported => &"Deserialize any not supported",
            Error::DeserializeIgnoredAnyNotSupported => &"Deserialize ignore any not supported",
            Error::IntegerTooLargeForBytes(_) => &"Integer too large for bytes"
        }
    }

    fn cause(&self) -> Option<&std::error::Error> {
        match self {
            Error::IntegerTooLargeForBytes(cause) => Some(cause),
            _ => None,
        }
    }
}
