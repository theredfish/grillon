use crate::{
    assertion::{
        traits::{IsEq, RangeInclusive},
        Assertion,
    },
    dsl::{is_between, Expression, Predicate, Range},
    StatusCode,
};
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

pub trait StatusCodeDslBis<T> {
    type Assertion;

    /// Evaluates the status assertion to run based on the [`Predicate`].
    fn eval(self, actual: T, predicate: Predicate) -> Self::Assertion;
}

impl StatusCodeDslBis<StatusCode> for StatusCode {
    type Assertion = Assertion<StatusCode>;

    fn eval(self, actual: StatusCode, predicate: Predicate) -> Self::Assertion {
        match predicate {
            Predicate::Is => self.is(actual),
            Predicate::IsNot => self.is_not(actual),
            _ => unimplemented!(),
        }
    }
}

impl StatusCodeDslBis<StatusCode> for u16 {
    type Assertion = Assertion<u16>;

    fn eval(self, actual: StatusCode, predicate: Predicate) -> Self::Assertion {
        match predicate {
            Predicate::Is => self.is(actual),
            Predicate::IsNot => self.is_not(actual),
            _ => unimplemented!(),
        }
    }
}

impl StatusCodeDslBis<StatusCode> for Range<StatusCode> {
    type Assertion = Assertion<StatusCode>;

    fn eval(self, actual: StatusCode, predicate: Predicate) -> Assertion<StatusCode> {
        match predicate {
            Predicate::Between => self.is_between(actual),
            _ => unimplemented!(
                "Invalid predicate for the status code(Range<StatusCode>) DSL : {predicate}"
            ),
        }
    }
}

impl StatusCodeDslBis<StatusCode> for Range<u16> {
    type Assertion = Assertion<u16>;

    fn eval(self, actual: StatusCode, predicate: Predicate) -> Assertion<u16> {
        match predicate {
            Predicate::Between => self.is_between(actual),
            _ => unimplemented!(
                "Invalid predicate for the status code(Range<u16>) DSL : {predicate}"
            ),
        }
    }
}

///
pub trait StatusCodeDslBisEquality<T>: StatusCodeDslBis<T>
where
    T: Debug,
    Self: Debug + Sized,
{
    fn is(self, actual: T) -> Self::Assertion;
    fn is_not(self, actual: T) -> Self::Assertion;
}

impl StatusCodeDslBisEquality<StatusCode> for StatusCode {
    fn is(self, actual: StatusCode) -> Self::Assertion {
        actual.is_eq(self)
    }

    fn is_not(self, actual: StatusCode) -> Self::Assertion {
        actual.is_eq(self)
    }
}

impl StatusCodeDslBisEquality<StatusCode> for u16 {
    fn is(self, actual: StatusCode) -> Self::Assertion {
        actual.is_eq(self)
    }

    fn is_not(self, actual: StatusCode) -> Self::Assertion {
        actual.is_eq(self)
    }
}

///
pub trait StatusCodeDslBisBetween<T>: StatusCodeDslBis<T>
where
    T: Debug,
    Self: Debug + Sized,
{
    fn is_between(self, actual: T) -> Self::Assertion;
}

impl StatusCodeDslBisBetween<StatusCode> for Range<StatusCode> {
    fn is_between(self, actual: StatusCode) -> Self::Assertion {
        actual.in_range(self.left, self.right)
    }
}

impl StatusCodeDslBisBetween<StatusCode> for Range<u16> {
    fn is_between(self, actual: StatusCode) -> Self::Assertion {
        actual.in_range(self.left, self.right)
    }
}
