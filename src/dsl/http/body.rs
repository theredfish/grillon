use std::path::PathBuf;

use crate::{
    assertion::traits::{Equality, JsonSchema},
    assertion::Assertion,
    dsl::expression::Predicate::{self, Is, IsNot, Schema},
    LogSettings,
};
use serde_json::Value;

/// Http json body DSL to assert body of a response.
pub trait JsonBodyDsl<T> {
    /// Asserts that the json response body is strictly equals to the provided value.
    fn is(&self, actual: T) -> Assertion<Value>;
    /// Asserts that the json response body is strictly not equals to the provided value.
    fn is_not(&self, actual: T) -> Assertion<Value>;
    /// Asserts that the json response body matches the json schema.
    fn schema(&self, schema: T) -> Assertion<Value>;
    /// Evaluates the json body assertion to run based on the [`Predicate`].
    fn eval(
        &self,
        actual: T,
        predicate: Predicate,
        log_settings: &LogSettings,
    ) -> Assertion<Value> {
        match predicate {
            Is => self.is(actual).assert(log_settings),
            IsNot => self.is_not(actual).assert(log_settings),
            Schema => self.schema(actual).assert(log_settings),
            _ => unimplemented!("Invalid predicate for the json body DSL: {predicate}"),
        }
    }
}

impl JsonBodyDsl<Value> for Value {
    fn is(&self, actual: Value) -> Assertion<Value> {
        actual.is_eq(self)
    }

    fn is_not(&self, actual: Value) -> Assertion<Value> {
        actual.is_ne(self)
    }

    fn schema(&self, actual: Value) -> Assertion<Value> {
        actual.matches_schema(self)
    }
}

impl JsonBodyDsl<Value> for &str {
    fn is(&self, actual: Value) -> Assertion<Value> {
        actual.is_eq(*self)
    }

    fn is_not(&self, actual: Value) -> Assertion<Value> {
        actual.is_ne(*self)
    }

    fn schema(&self, actual: Value) -> Assertion<Value> {
        actual.matches_schema(*self)
    }
}

impl JsonBodyDsl<Value> for String {
    fn is(&self, actual: Value) -> Assertion<Value> {
        actual.is_eq(self)
    }

    fn is_not(&self, actual: Value) -> Assertion<Value> {
        actual.is_ne(self)
    }

    fn schema(&self, actual: Value) -> Assertion<Value> {
        actual.matches_schema(self)
    }
}

impl JsonBodyDsl<Value> for PathBuf {
    fn is(&self, actual: Value) -> Assertion<Value> {
        actual.is_eq(self)
    }

    fn is_not(&self, actual: Value) -> Assertion<Value> {
        actual.is_ne(self)
    }

    fn schema(&self, actual: Value) -> Assertion<Value> {
        actual.matches_schema(self)
    }
}
