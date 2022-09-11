//! Module containing all the different parts we can assert against. These parts
//! are also used to build assertion messages in a convenient way.

use serde::Deserialize;
use strum::Display;

/// Represents all the parts we can assert against. Provides a string
/// representation for each variant to build assertion messages in a convenient
/// way.
#[derive(Display, Deserialize, PartialEq, Eq, Debug)]
pub enum Part {
    /// The json body of an http response.
    #[strum(serialize = "json body")]
    #[serde(rename = "json body")]
    JsonBody,
    /// The headers in an http response.
    #[strum(serialize = "headers")]
    #[serde(rename = "headers")]
    Headers,
    /// A header in an http response.
    #[strum(serialize = "header")]
    #[serde(rename = "header")]
    Header,
    /// The status code of an http response.
    #[strum(serialize = "status code")]
    #[serde(rename = "status code")]
    StatusCode,
    /// The response time of an http response.
    #[strum(serialize = "response time")]
    #[serde(rename = "response time")]
    ResponseTime,
}

#[cfg(test)]
pub mod tests {
    use super::Part;
    use serde_json::Value;
    use test_case::test_case;

    #[test_case(Value::String(String::from("json body")), Part::JsonBody; "Failed to deserialize part JsonBody")]
    #[test_case(Value::String(String::from("headers")), Part::Headers; "Failed to deserialize part Headers")]
    #[test_case(Value::String(String::from("header")), Part::Header; "Failed to deserialize part Header")]
    #[test_case(Value::String(String::from("status code")), Part::StatusCode; "Failed to deserialize part StatusCode")]
    #[test_case(Value::String(String::from("response time")), Part::ResponseTime; "Failed to deserialize part ResponseTime")]
    fn deser_part(json_part: Value, part: Part) {
        assert_eq!(serde_json::from_value::<Part>(json_part).unwrap(), part)
    }
}
