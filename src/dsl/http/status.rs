//! The `http::status` DSL provides built-in functions to perform declarative
//! assertions against the status of an http response.
use crate::{
    // assert::{AssertBool, AssertEq, AssertNe, Assertion},
    assertion::{
        traits::{IsEq, IsNe, RangeInclusive},
        Assertion,
    },
    dsl::{
        expression::{
            Predicate::{self, Between, Is, IsNot},
            Range,
        },
        is_between,
        part::Part,
        Expression,
    },
};
use http::StatusCode;
use std::fmt::Debug;

/// A short-hand function to test if the status code
/// of the response is in the range of 2xx codes.
pub fn is_success() -> Expression<Range<u16>> {
    is_between(200, 299)
}

/// A short-hand function to test if the status code
/// of the response is in the range of 4xx codes.
pub fn is_client_error() -> Expression<Range<u16>> {
    is_between(400, 499)
}

/// A short-hand function to test if the status code
/// of the response is in the range of 5xx codes.
pub fn is_server_error() -> Expression<Range<u16>> {
    is_between(500, 599)
}

/// Http status DSL to assert the status code of a response.
///
/// ```rust
/// use grillon::{Result, Grillon, StatusCode};
/// use grillon::dsl::{is, is_between, is_not, http::is_success};
///
/// #[tokio::test]
/// async fn check_status() -> Result<()> {
///    Grillon::new("http://jsonplaceholder.typicode.com")?
///        .get("users/1")
///        .assert()
///        .await
///        .status(is(200))
///        .status(is(StatusCode::OK))
///        .status(is_not(500))
///        .status(is_not(StatusCode::INTERNAL_SERVER_ERROR))
///        .status(is_success())
///        .status(is_between(200, 204))
///        .status(is_between(StatusCode::OK, StatusCode::NO_CONTENT));
///
///    Ok(())
/// }

pub trait StatusCodeDsl<T>
where
    T: Debug,
    Self: Debug + Sized,
{
    /// Evaluates the status assertion to run based on the [`Predicate`].
    fn eval(self, actual: T, predicate: Predicate) -> Assertion<Self>;
}

/// Http status DSL to assert the status code of a response is in the
/// given range.
pub trait StatusCodeRange<T>: StatusCodeDsl<T>
where
    T: Debug,
    Self: Debug + Sized,
{
    fn is_between(self, actual: T) -> Assertion<T>;
}

/// Http status DSL to assert the status code equality of a response.
pub trait StatusCodeEquality<T>: StatusCodeDsl<T>
where
    T: Debug,
    Self: Debug + Sized,
{
    fn is(self, actual: T) -> Assertion<Self>;
    fn is_not(self, actual: T) -> Assertion<Self>;
}

impl StatusCodeDsl<StatusCode> for u16 {
    fn eval(self, actual: StatusCode, predicate: Predicate) -> Assertion<u16> {
        match predicate {
            Predicate::Is => self.is(actual),
            Predicate::IsNot => self.is_not(actual),
            _ => unimplemented!("Invalid predicate for the status code(u16) DSL : {predicate}"),
        }
    }
}

impl StatusCodeDsl<StatusCode> for StatusCode {
    fn eval(self, actual: StatusCode, predicate: Predicate) -> Assertion<StatusCode> {
        match predicate {
            Predicate::Is => self.is(actual),
            Predicate::IsNot => self.is_not(actual),
            _ => unimplemented!(
                "Invalid predicate for the status code(StatusCode) DSL : {predicate}"
            ),
        }
    }
}

impl StatusCodeDsl<StatusCode> for Range<u16> {
    fn eval(self, actual: StatusCode, predicate: Predicate) -> Assertion<u16> {
        match predicate {
            Predicate::Between => self.is_between(actual),
            _ => unimplemented!(
                "Invalid predicate for the status code(Range<u16>) DSL : {predicate}"
            ),
        }
    }
}

impl StatusCodeDsl<StatusCode> for Range<StatusCode> {
    fn eval(self, actual: StatusCode, predicate: Predicate) -> Assertion<StatusCode> {
        match predicate {
            Predicate::Between => self.is_between(actual),
            _ => unimplemented!(
                "Invalid predicate for the status code(Range<StatusCode>) DSL : {predicate}"
            ),
        }
    }
}

impl StatusCodeEquality<StatusCode> for u16 {
    fn is(self, actual: StatusCode) -> Assertion<u16> {
        actual.is_eq(self)
    }

    fn is_not(self, actual: StatusCode) -> Assertion<u16> {
        actual.is_ne(self)
    }
}

impl StatusCodeEquality<StatusCode> for StatusCode {
    fn is(self, actual: StatusCode) -> Assertion<StatusCode> {
        actual.is_eq(self)
    }

    fn is_not(self, actual: StatusCode) -> Assertion<StatusCode> {
        actual.is_ne(self)
    }
}

impl StatusCodeRange<StatusCode> for Range<u16> {
    fn is_between(self, actual: StatusCode) -> Assertion<u16> {
        let (min, max) = (self.left, self.right);
        actual.in_range(min, max)
    }
}

impl StatusCodeRange<StatusCode> for Range<StatusCode> {
    fn is_between(self, actual: StatusCode) -> Assertion<StatusCode> {
        let (min, max) = (self.left, self.right);
        actual.in_range(min, max)
    }
}
