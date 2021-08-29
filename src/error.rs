use http::uri::InvalidUri;
use std::error::Error as StdError;
use std::fmt;

pub type Result<T> = std::result::Result<T, Error>;

#[derive(Debug)]
pub enum Error {
    UriParseError(InvalidUri),
}

macro_rules! impl_from_error {
    ($f:ty, $e:expr) => {
        impl From<$f> for Error {
            fn from(f: $f) -> Error {
                $e(f)
            }
        }
    };
}

impl_from_error!(InvalidUri, Error::UriParseError);

impl StdError for Error {}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        use crate::Error::*;

        match *self {
            UriParseError(ref err) => fmt::Display::fmt(err, f),
        }
    }
}
