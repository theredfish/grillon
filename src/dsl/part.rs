//! Module containing all the different parts we can assert against. These parts
//! are also used to build assertion messages in a convenient way.

use strum::Display;

/// Represents all the parts we can assert against. Provides a string
/// representation for each variant to build assertion messages in a convenient
/// way.
#[derive(Display)]
pub enum Part {
    /// The json body of an http response.
    #[strum(serialize = "json body")]
    JsonBody,
    /// The headers in an http response.
    #[strum(serialize = "headers")]
    Headers,
    /// A header in an http response.
    #[strum(serialize = "header")]
    Header,
    /// The status code of an http response.
    #[strum(serialize = "status code")]
    StatusCode,
    /// The response time of an http response.
    #[strum(serialize = "response time")]
    ResponseTime,
}
