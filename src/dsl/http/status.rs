//! The `http::status` DSL provides built-in functions to perform declarative
//! assertions against the status of an http response.
use crate::{
    assert::{AssertBool, AssertEq, AssertNe, Assertion},
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

pub trait StatusCodeDsl<T> {
    /// Evaluates the status assertion to run based on the [`Predicate`].
    fn eval(&self, actual: T, predicate: Predicate) -> Assertion;
}

/// Http status DSL to assert the status code of a response is in the
/// given range.
pub trait StatusCodeRange<T>: StatusCodeDsl<T> {
    fn is_between(&self, actual: T) -> Assertion;
}

/// Http status DSL to assert the status code equality of a response.
pub trait StatusCodeEquality<T>: StatusCodeDsl<T> {
    fn is(&self, actual: T) -> Assertion;
    fn is_not(&self, actual: T) -> Assertion;
}

impl StatusCodeDsl<StatusCode> for u16 {
    fn eval(&self, actual: StatusCode, predicate: Predicate) -> Assertion {
        match predicate {
            Predicate::Is => self.is(actual),
            Predicate::IsNot => self.is_not(actual),
            _ => unimplemented!("Invalid predicate for the status code(u16) DSL : {predicate}"),
        }
    }
}

impl StatusCodeDsl<StatusCode> for StatusCode {
    fn eval(&self, actual: StatusCode, predicate: Predicate) -> Assertion {
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
    fn eval(&self, actual: StatusCode, predicate: Predicate) -> Assertion {
        match predicate {
            Predicate::Between => self.is_between(actual),
            _ => unimplemented!(
                "Invalid predicate for the status code(Range<u16>) DSL : {predicate}"
            ),
        }
    }
}

impl StatusCodeDsl<StatusCode> for Range<StatusCode> {
    fn eval(&self, actual: StatusCode, predicate: Predicate) -> Assertion {
        match predicate {
            Predicate::Between => self.is_between(actual),
            _ => unimplemented!(
                "Invalid predicate for the status code(Range<StatusCode>) DSL : {predicate}"
            ),
        }
    }
}

impl StatusCodeEquality<StatusCode> for u16 {
    fn is(&self, actual: StatusCode) -> Assertion {
        let expected = StatusCode::from_u16(*self).unwrap();

        let ty = AssertEq {
            left: actual,
            right: expected,
        };

        Assertion::new(Box::new(ty), Is, Part::StatusCode)
    }

    fn is_not(&self, actual: StatusCode) -> Assertion {
        let expected = StatusCode::from_u16(*self).unwrap();

        let ty = AssertNe {
            left: actual,
            right: expected,
        };

        Assertion::new(Box::new(ty), IsNot, Part::StatusCode)
    }
}

impl StatusCodeEquality<StatusCode> for StatusCode {
    fn is(&self, actual: StatusCode) -> Assertion {
        let ty = AssertEq {
            left: actual,
            right: *self,
        };

        Assertion::new(Box::new(ty), Is, Part::StatusCode)
    }

    fn is_not(&self, actual: StatusCode) -> Assertion {
        let ty = AssertNe {
            left: actual,
            right: *self,
        };

        Assertion::new(Box::new(ty), IsNot, Part::StatusCode)
    }
}

impl StatusCodeRange<StatusCode> for Range<u16> {
    fn is_between(&self, actual: StatusCode) -> Assertion {
        let (min, max) = (self.left, self.right);
        let actual = actual.as_u16();
        let result = actual >= min && actual <= max;

        let ty = AssertBool {
            left: actual,
            right: format!("[{min},{max}]"),
            result,
        };

        Assertion::new(Box::new(ty), Between, Part::ResponseTime)
    }
}

impl StatusCodeRange<StatusCode> for Range<StatusCode> {
    fn is_between(&self, actual: StatusCode) -> Assertion {
        let (min, max) = (self.left.as_u16(), self.right.as_u16());
        let actual = actual.as_u16();
        let result = actual >= min && actual <= max;

        let ty = AssertBool {
            left: actual,
            right: format!("[{min},{max}]"),
            result,
        };

        Assertion::new(Box::new(ty), Between, Part::ResponseTime)
    }
}