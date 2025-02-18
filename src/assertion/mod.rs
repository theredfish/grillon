//! Functionnality for asserting.
//!
//! This module contains a set of structures, types and implementations to
//! create expressive assertions decoupled from the DSL. This is ideal for
//! external implementations.
//!
//! This is not generally used by end users, instead the [`dsl`] module should
//! provide the built-in functions served as part of the library.
//!
//! The left and right hands of an [`Assertion`] enforce the implementation of
//! [`Debug`] and [`Serialize`]. This is because the library can produce
//! different types of logs to standard output : human-readable
//! (debuggable) and json formats.
//!
//! [`dsl`]: crate::dsl

mod impls;
#[allow(clippy::wrong_self_convention)]
pub mod traits;

use crate::{
    dsl::{Part, Predicate},
    grillon::LogSettings,
};
use serde::Serialize;
use serde_json::{json, Value};
use std::any::Any;
use std::fmt::Debug;
use strum::Display;

/// Short-hand types and aliases used for assertions.
pub mod types {
    use http::{header::HeaderName, HeaderValue};

    /// An alias to manipulate an internal representation of headers as tuples
    /// of strings.
    pub type Headers = Vec<(String, String)>;
    /// An alias to manipulate an internal representation of headers as tuples
    /// of [`HeaderName`] and [`HeaderValue`].
    pub type HeaderTupleVec = Vec<(HeaderName, HeaderValue)>;
    /// An alias to manipulate an internal representation of headers as tuples
    /// of str.
    pub type HeaderStrTupleVec = Vec<(&'static str, &'static str)>;
    /// An alias to manipulate an internal representation of a header as a
    /// `String`.
    pub type Header = String;
}

/// Represents left or right hands in an [`Assertion`].
#[derive(Serialize, Debug)]
#[serde(untagged)]
pub enum Hand<T>
where
    T: Debug,
{
    /// The left hand of the assertion.
    Left(T),
    /// The right hand of the assertion.
    Right(T),
    /// A hand composed of two elements.
    Compound(T, T),
    /// An empty hand
    Empty,
}

/// The assertion encapsulating information about the [`Part`] under
/// test, the [`Predicate`] used, the [`AssertionResult`] and the right and left
/// [`Hand`]s.
#[derive(Serialize, Debug)]
pub struct Assertion<T>
where
    T: Debug + Serialize,
{
    /// The part under test.
    pub part: Part,
    /// The predicate applied in the test.
    pub predicate: Predicate,
    /// The left hand of the assertion.
    pub left: Hand<T>,
    /// The right hand of the assertion.
    pub right: Hand<T>,
    /// The assertion result.
    pub result: AssertionResult,
}

/// Unprocessable event reason. This enum should
/// be used when the assertion syntax is correct
/// but the implementor is unable to process the
/// assertion due to an unexpected event.
///
/// For example, when an implementation asserts
/// that a word exists in a file but there is no
/// read access. In this case, the assertion
/// fails not because the word is missing, but
/// because the file content cannot be
/// processed.
#[derive(Serialize, Debug)]
#[serde(rename_all = "snake_case")]
pub enum UnprocessableReason {
    /// Unprocessable json path with the string representation of the path.
    InvalidJsonPath(String),
    /// Unprocessable json body because it's missing.
    MissingJsonBody,
    /// Unprocessable header value because the correspond header key is missing.
    MissingHeader,
    /// Unprocessable json schema.
    InvalidJsonSchema(String, String),
    /// Serialization failure.
    SerializationFailure(String),
    /// Invalid HTTP request headers.
    InvalidHttpRequestHeaders(String),
    /// Invalid HTTP header value.
    InvalidHeaderValue(String),
    /// Invalid regex pattern.
    InvalidRegex(String),
    /// If the HTTP request results in an error while sending request, redirect
    /// loop was detected or redirect limit was exhausted.
    HttpRequestFailure(String),
    /// Unprocessable entity.
    Other(String),
}

// Strum cannot be used here since sum type fields are
// not supported yet just like positional arguments for
// tuple variants.
impl std::fmt::Display for UnprocessableReason {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            UnprocessableReason::InvalidJsonPath(message) => {
                write!(f, "Unprocessable json path: {message}")
            }
            UnprocessableReason::MissingJsonBody => {
                write!(f, "Unprocessable json body: missing")
            }
            UnprocessableReason::MissingHeader => {
                write!(f, "Unprocessable header: header key is missing")
            }
            UnprocessableReason::InvalidJsonSchema(schema, instance) => {
                write!(f, "Invalid json schema: {schema} => {instance}")
            }
            UnprocessableReason::SerializationFailure(message) => {
                write!(f, "Serialization failure: {message}")
            }
            UnprocessableReason::InvalidHttpRequestHeaders(details) => {
                write!(f, "Invalid HTTP request headers: {details}")
            }
            UnprocessableReason::InvalidHeaderValue(details) => {
                write!(f, "Invalid HTTP response header value: {details}")
            }
            UnprocessableReason::InvalidRegex(regex) => {
                write!(f, "Invalid regex pattern: {regex}")
            }
            UnprocessableReason::HttpRequestFailure(details) => {
                write!(f, "Http request failure: {details}")
            }
            UnprocessableReason::Other(message) => write!(f, "{message}"),
        }
    }
}

/// The assertion's result.
#[derive(Serialize, Display, Debug)]
#[serde(rename_all = "snake_case")]
#[strum(serialize_all = "snake_case")]
pub enum AssertionResult {
    /// When the assertion passed.
    Passed,
    /// When the assertion failed.
    Failed,
    /// When the assertion didn't start.
    NotYetStarted,
    /// When the assertion is correct but cannot be processed
    /// due to an unexpected reason.
    Unprocessable(UnprocessableReason),
}

/// Represents an assertion log.
///
/// A log is built according to this scheme:
/// - part: \<part\> \[compound_hand_part]
/// - \<predicate\>: \<expected_value\>
/// - was: \<found_value\> (only in case of failure)
///
/// The log will be displayed for both [`LogSettings::StdOutput`] and
/// [`LogSettings::StdAssert`]
pub struct AssertionLog(String);

impl AssertionLog {
    /// Builds the assertion message based on the [`Predicate`], the [`Part`]
    /// and the [`AssertionResult`].
    pub fn new<T: Any + Debug + Serialize>(assertion: &Assertion<T>) -> Self {
        if let AssertionResult::Unprocessable(reason) = &assertion.result {
            return Self(format!("{reason}"));
        }

        match assertion.part {
            Part::JsonPath => Self::jsonpath_log(assertion),
            _ => Self::log(assertion),
        }
    }

    fn log<T: Debug + Serialize>(assertion: &Assertion<T>) -> Self {
        let predicate = &assertion.predicate;
        let part = &assertion.part;

        let left = match &assertion.left {
            Hand::Left(left) => format!("{left:#?}"),
            Hand::Compound(left, right) if part == &Part::StatusCode => {
                format!("{left:#?} and {right:#?}")
            }
            _ => "Unexpected left hand in right hand".to_string(),
        };
        let right = match &assertion.right {
            Hand::Right(right) => format!("{right:#?}"),
            Hand::Compound(left, right) if part == &Part::StatusCode => {
                format!("{left:#?} and {right:#?}")
            }
            _ => "Unexpected left hand in right hand".to_string(),
        };

        let result = &assertion.result;
        let part = format!("part: {part}");
        let message = match result {
            AssertionResult::Passed => format!(
                "result: {result}
{part}
{predicate}: {right}"
            ),
            AssertionResult::Failed => format!(
                "result: {result}
{part}
{predicate}: {right}
was: {left}"
            ),
            AssertionResult::NotYetStarted => format!("Not yet started : {part}"),
            AssertionResult::Unprocessable(reason) => format!("{reason}"),
        };

        Self(message)
    }

    fn jsonpath_log<T: Any + Debug + Serialize>(assertion: &Assertion<T>) -> Self {
        let predicate = &assertion.predicate;
        let part = &assertion.part;

        let left_hand = match &assertion.left {
            Hand::Compound(left, right) if part == &Part::JsonPath => (left, right),
            _ => return Self("<unexpected left hand>".to_string()),
        };
        let right_hand = match &assertion.right {
            Hand::Right(right) if part == &Part::JsonPath => right,
            _ => return Self("<unexpected right hand>".to_string()),
        };

        let jsonpath = left_hand.0;
        #[allow(trivial_casts)]
        let jsonpath = match (jsonpath as &dyn Any).downcast_ref::<Value>() {
            Some(Value::String(jsonpath_string)) => jsonpath_string.to_string(),
            _ => format!("{jsonpath:?}"),
        };

        let jsonpath_value = left_hand.1;

        let result = &assertion.result;
        let part = format!("part: {part} '{jsonpath}'");
        let message = match result {
            AssertionResult::Passed => format!(
                "result: {result}
{part}
{predicate}: {right_hand:#?}"
            ),
            AssertionResult::Failed => format!(
                "result: {result}
{part}
{predicate}: {right_hand:#?}
was: {jsonpath_value:#?}"
            ),
            AssertionResult::NotYetStarted => format!("[Not yet started] {part}"),
            AssertionResult::Unprocessable(reason) => format!("{reason}"),
        };

        Self(message)
    }
}

impl<T> Assertion<T>
where
    T: Debug + Serialize + 'static,
{
    /// Returns if the assertion passed.
    pub fn passed(&self) -> bool {
        matches!(self.result, AssertionResult::Passed)
    }

    /// Returns if the assertion failed.
    pub fn failed(&self) -> bool {
        matches!(
            self.result,
            AssertionResult::Failed | AssertionResult::Unprocessable(_)
        )
    }

    /// Runs the assertion and produce the the result results with the given
    /// [`LogSettings`].
    pub fn assert(self, log_settings: &LogSettings) -> Assertion<T> {
        let message = self.log();
        match log_settings {
            LogSettings::StdOutput => println!("\n{message}"),
            LogSettings::StdAssert => assert!(self.passed(), "\n\n{message}"),
            LogSettings::JsonOutput => {
                let json = serde_json::to_string(&json!(self))
                    .expect("Unexpected json failure: failed to serialize assertion");
                println!("{json}");
            }
        }

        self
    }

    fn log(&self) -> String {
        AssertionLog::new(self).0
    }
}

impl From<bool> for AssertionResult {
    fn from(val: bool) -> Self {
        if val {
            return AssertionResult::Passed;
        }

        AssertionResult::Failed
    }
}

#[cfg(test)]
mod tests {
    use super::{AssertionResult, Hand};
    use crate::dsl::Predicate::{Between, LessThan};
    use crate::{assertion::Assertion, dsl::Part};
    use serde_json::json;

    #[test]
    fn it_should_serialize_status_code() {
        let assertion: Assertion<u16> = Assertion {
            part: Part::StatusCode,
            predicate: Between,
            left: Hand::Left(200),
            right: Hand::Compound(200, 299),
            result: AssertionResult::Passed,
        };

        let expected_json = json!({
            "part": "status code",
            "predicate": "should be between",
            "left": 200,
            "right": [200, 299],
            "result": "passed"
        });

        assert_eq!(json!(assertion), expected_json);
    }

    #[test]
    fn it_should_serialize_failed_response_time() {
        let assertion: Assertion<u64> = Assertion {
            part: Part::ResponseTime,
            predicate: LessThan,
            left: Hand::Left(300),
            right: Hand::Right(248),
            result: AssertionResult::Failed,
        };

        let expected_json = json!({
            "part": "response time",
            "predicate": "should be less than",
            "left": 300,
            "right": 248,
            "result": "failed"
        });

        assert_eq!(json!(assertion), expected_json);
    }
}
