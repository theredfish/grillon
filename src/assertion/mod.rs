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
use serde_json::json;
use std::fmt::Debug;

/// Short-hand types and aliases used for assertions.
pub mod types {
    use http::{header::HeaderName, HeaderValue};

    /// An alias to manipulate an internal representation of headers as tuples
    /// of strings.
    pub type Headers = Vec<(String, String)>;
    /// An alias to manipulate an internal representation of headers as tuples
    /// of [`HeaderName`] and [`HeaderValue`].
    pub type HeaderTupleVec = Vec<(HeaderName, HeaderValue)>;
}

/// Represents left or right hands in an [`Assertion`].
#[derive(Serialize)]
#[serde(untagged)]
pub enum Hand<T>
where
    T: Debug,
{
    /// The left hand of the assertion.
    Left(T),
    /// The right hand of the assertion.
    Right(T),
    /// A more complex hand made of a range that can be left or right.
    Range(T, T),
}

/// The assertion encapsulating information about the [`Part`] under
/// test, the [`Predicate`] used, the [`AssertionResult`] and the right and left
/// [`Hand`]s.
#[derive(Serialize)]
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

/// The assertion's result.
#[derive(Serialize)]
#[serde(rename_all = "snake_case")]
pub enum AssertionResult {
    /// When the assertion passed.
    Passed,
    /// When the assertion failed.
    Failed,
    /// When the assertion didn't start.
    NotYetStarted,
}

impl<T> Assertion<T>
where
    T: Debug + Serialize,
{
    /// Returns if the assertion passed.
    pub fn passed(&self) -> bool {
        match self.result {
            AssertionResult::Passed => true,
            AssertionResult::Failed | AssertionResult::NotYetStarted => false,
        }
    }

    /// Returns if the assertion failed.
    pub fn failed(&self) -> bool {
        match self.result {
            AssertionResult::Failed => true,
            AssertionResult::Passed | AssertionResult::NotYetStarted => false,
        }
    }

    /// Runs the assertion and produce the the result results with the given
    /// [`LogSettings`].
    pub fn assert(self, log_settings: &LogSettings) -> Assertion<T> {
        let message = self.message();
        match log_settings {
            LogSettings::StdOut => println!("{message}"),
            LogSettings::StdAssert => assert!(self.passed(), "{}", message),
            LogSettings::Json => {
                let json = serde_json::to_string(&json!(self))
                    .expect("Unexpected json failure: failed to serialize assertion");
                println!("{json}");
            }
        }

        self
    }

    /// Builds the assertion message based on the [`Predicate`], the [`Part`]
    /// and the [`AssertionResult`].
    fn message(&self) -> String {
        let result = &self.result;

        let predicate = &self.predicate;
        let part = &self.part;
        let left = match &self.left {
            Hand::Left(left) => format!("{left:#?}"),
            Hand::Range(min, max) => format!("{min:#?} and {max:#?}"),
            Hand::Right(_) => "".to_string(),
        };
        let right = match &self.right {
            Hand::Right(right) => format!("{right:#?}"),
            Hand::Range(min, max) => format!("{min:#?} and {max:#?}"),
            Hand::Left(_) => "".to_string(),
        };

        // The base message is built as a passing case.
        let message = match part {
            Part::Empty => format!("{left} {predicate} {right}"),
            _ => format!("{part} {predicate} {right}"),
        };

        match result {
            AssertionResult::Passed => message,
            AssertionResult::Failed => format!("{message}. Found {left}"),
            AssertionResult::NotYetStarted => format!("Not yet started : {message}"),
        }
    }
}

impl From<bool> for AssertionResult {
    fn from(val: bool) -> Self {
        if val {
            AssertionResult::Passed
        } else {
            AssertionResult::Failed
        }
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
            right: Hand::Range(200, 299),
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
