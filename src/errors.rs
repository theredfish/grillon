use std::error::Error;
use std::fmt;

#[derive(Debug)]
pub enum ApiHoursError {
    IncompleteHttRequest,
}

impl Error for ApiHoursError {}

impl fmt::Display for ApiHoursError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            ApiHoursError::IncompleteHttRequest => write!(
                f,
                "Please verify your http request, required parts are missing."
            ),
        }
    }
}
