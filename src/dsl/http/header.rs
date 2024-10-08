use http::HeaderValue;

use crate::{
    assertion::{traits::Equality, types::Header, Assertion},
    dsl::expression::Predicate,
    LogSettings,
};

/// Http header DSL to assert a single header from a response.
pub trait HeaderDsl<T> {
    /// Asserts the header is strictly equal to the provided ones.
    fn is(&self, actual: T) -> Assertion<Header>;
    /// Asserts the header is strictly not equal to the provided ones.
    fn is_not(&self, actual: T) -> Assertion<Header>;
    /// Evaluates the header assertion to run based on the [`Predicate`].
    fn eval(
        &self,
        actual: T,
        predicate: Predicate,
        log_settings: &LogSettings,
    ) -> Assertion<Header> {
        match predicate {
            Predicate::Is => self.is(actual).assert(log_settings),
            Predicate::IsNot => self.is_not(actual).assert(log_settings),
            _ => unimplemented!("Invalid predicate for the header DSL: {predicate}"),
        }
    }
}

impl HeaderDsl<HeaderValue> for &str {
    fn is(&self, actual: HeaderValue) -> Assertion<Header> {
        actual.is_eq(self)
    }

    fn is_not(&self, actual: HeaderValue) -> Assertion<Header> {
        actual.is_ne(self)
    }
}

impl HeaderDsl<HeaderValue> for String {
    fn is(&self, actual: HeaderValue) -> Assertion<Header> {
        actual.is_eq(self)
    }

    fn is_not(&self, actual: HeaderValue) -> Assertion<Header> {
        actual.is_ne(self)
    }
}

impl HeaderDsl<HeaderValue> for HeaderValue {
    fn is(&self, actual: HeaderValue) -> Assertion<Header> {
        actual.is_eq(self)
    }

    fn is_not(&self, actual: HeaderValue) -> Assertion<Header> {
        actual.is_ne(self)
    }
}
