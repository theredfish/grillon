//! The json path domain-specific language.

use crate::{
    assertion::traits::{Equality, JsonSchema},
    assertion::Assertion,
    dsl::expression::Predicate::{self, Is, IsNot, Schema},
    LogSettings,
};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::path::PathBuf;

/// Represents the result of a json path query.
///
/// This structure is used to wrap the json path result
/// and run assertions against.
#[derive(Serialize, Deserialize, Debug, PartialEq, Eq)]
pub struct JsonPathResult<'p, T> {
    /// The path used to run the json path query.
    pub path: &'p str,
    /// The resulting json value of the json path query.
    pub value: T,
}

impl<'p, T> JsonPathResult<'p, T> {
    /// Creates a new instance of `JsonPathResult` that is
    /// wrapping the given `path` and the given `value`.
    pub fn new(path: &'p str, value: T) -> Self {
        Self { path, value }
    }
}

/// Http json body DSL to assert body of a response.
pub trait JsonPathDsl<T> {
    /// Asserts that the json response body is strictly equals to the provided value.
    fn is(&self, jsonpath_res: JsonPathResult<'_, T>) -> Assertion<Value>;
    /// Asserts that the json response body is strictly not equals to the provided value.
    fn is_not(&self, jsonpath_res: JsonPathResult<'_, T>) -> Assertion<Value>;
    /// Asserts that the value of the json path matches the json schema.
    fn schema(&self, jsonpath_res: JsonPathResult<'_, T>) -> Assertion<Value>;
    /// Evaluates the json body assertion to run based on the [`Predicate`].
    fn eval(
        &self,
        jsonpath_res: JsonPathResult<'_, T>,
        predicate: Predicate,
        log_settings: &LogSettings,
    ) -> Assertion<Value> {
        match predicate {
            Is => self.is(jsonpath_res).assert(log_settings),
            IsNot => self.is_not(jsonpath_res).assert(log_settings),
            Schema => self.schema(jsonpath_res).assert(log_settings),
            _ => unimplemented!("Invalid predicate for the json path DSL: {predicate}"),
        }
    }
}

impl JsonPathDsl<Value> for Value {
    fn is(&self, jsonpath_res: JsonPathResult<'_, Value>) -> Assertion<Value> {
        jsonpath_res.is_eq(self)
    }

    fn is_not(&self, jsonpath_res: JsonPathResult<'_, Value>) -> Assertion<Value> {
        jsonpath_res.is_ne(self)
    }

    fn schema(&self, jsonpath_res: JsonPathResult<'_, Value>) -> Assertion<Value> {
        jsonpath_res.matches_schema(self)
    }
}

impl JsonPathDsl<Value> for String {
    fn is(&self, jsonpath_res: JsonPathResult<'_, Value>) -> Assertion<Value> {
        jsonpath_res.is_eq(self)
    }

    fn is_not(&self, jsonpath_res: JsonPathResult<'_, Value>) -> Assertion<Value> {
        jsonpath_res.is_ne(self)
    }

    fn schema(&self, jsonpath_res: JsonPathResult<'_, Value>) -> Assertion<Value> {
        jsonpath_res.matches_schema(self)
    }
}

impl JsonPathDsl<Value> for &str {
    fn is(&self, jsonpath_res: JsonPathResult<'_, Value>) -> Assertion<Value> {
        jsonpath_res.is_eq(*self)
    }

    fn is_not(&self, jsonpath_res: JsonPathResult<'_, Value>) -> Assertion<Value> {
        jsonpath_res.is_ne(*self)
    }

    fn schema(&self, jsonpath_res: JsonPathResult<'_, Value>) -> Assertion<Value> {
        jsonpath_res.matches_schema(*self)
    }
}

impl JsonPathDsl<Value> for PathBuf {
    fn is(&self, jsonpath_res: JsonPathResult<'_, Value>) -> Assertion<Value> {
        jsonpath_res.is_eq(self)
    }

    fn is_not(&self, jsonpath_res: JsonPathResult<'_, Value>) -> Assertion<Value> {
        jsonpath_res.is_ne(self)
    }

    fn schema(&self, jsonpath_res: JsonPathResult<'_, Value>) -> Assertion<Value> {
        jsonpath_res.matches_schema(self)
    }
}
