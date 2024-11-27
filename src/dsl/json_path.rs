//! The json path domain-specific language.

use crate::{
    assertion::{
        traits::{Container, Equality, JsonSchema, Matching},
        Assertion,
    },
    dsl::expression::Predicate::{
        self, Contains, DoesNotContain, DoesNotMatch, Is, IsNot, Matches, Schema,
    },
    LogSettings,
};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::path::PathBuf;

use super::RegexWrapper;

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

/// Json path DSL to assert a value at a given path.
pub trait JsonPathDsl<T> {
    /// Evaluates the status assertion to run depending on the [`Predicate`].
    /// The test results will be produced on the given output configured via the
    /// [`LogSettings`].
    fn eval(
        &self,
        actual: JsonPathResult<'_, Value>,
        predicate: Predicate,
        log_settings: &LogSettings,
    ) -> Assertion<Value>;
}

impl JsonPathDsl<Value> for Value {
    fn eval(
        &self,
        jsonpath_res: JsonPathResult<'_, Value>,
        predicate: Predicate,
        log_settings: &LogSettings,
    ) -> Assertion<Value> {
        match predicate {
            Is => self.is(jsonpath_res).assert(log_settings),
            IsNot => self.is_not(jsonpath_res).assert(log_settings),
            Schema => self.schema(jsonpath_res).assert(log_settings),
            Contains => self.contains(jsonpath_res).assert(log_settings),
            DoesNotContain => self.does_not_contain(jsonpath_res).assert(log_settings),
            _ => unimplemented!("Invalid predicate for the json path DSL: {predicate}"),
        }
    }
}

impl JsonPathDsl<Value> for String {
    fn eval(
        &self,
        jsonpath_res: JsonPathResult<'_, Value>,
        predicate: Predicate,
        log_settings: &LogSettings,
    ) -> Assertion<Value> {
        match predicate {
            Is => self.is(jsonpath_res).assert(log_settings),
            IsNot => self.is_not(jsonpath_res).assert(log_settings),
            Schema => self.schema(jsonpath_res).assert(log_settings),
            Contains => self.contains(jsonpath_res).assert(log_settings),
            DoesNotContain => self.does_not_contain(jsonpath_res).assert(log_settings),
            _ => unimplemented!("Invalid predicate for the json path DSL: {predicate}"),
        }
    }
}

impl JsonPathDsl<Value> for &str {
    fn eval(
        &self,
        jsonpath_res: JsonPathResult<'_, Value>,
        predicate: Predicate,
        log_settings: &LogSettings,
    ) -> Assertion<Value> {
        match predicate {
            Is => self.is(jsonpath_res).assert(log_settings),
            IsNot => self.is_not(jsonpath_res).assert(log_settings),
            Schema => self.schema(jsonpath_res).assert(log_settings),
            Contains => self.contains(jsonpath_res).assert(log_settings),
            DoesNotContain => self.does_not_contain(jsonpath_res).assert(log_settings),
            _ => unimplemented!("Invalid predicate for the json path DSL: {predicate}"),
        }
    }
}

impl JsonPathDsl<Value> for PathBuf {
    fn eval(
        &self,
        jsonpath_res: JsonPathResult<'_, Value>,
        predicate: Predicate,
        log_settings: &LogSettings,
    ) -> Assertion<Value> {
        match predicate {
            Is => self.is(jsonpath_res).assert(log_settings),
            IsNot => self.is_not(jsonpath_res).assert(log_settings),
            Schema => self.schema(jsonpath_res).assert(log_settings),
            Contains => self.contains(jsonpath_res).assert(log_settings),
            DoesNotContain => self.does_not_contain(jsonpath_res).assert(log_settings),
            _ => unimplemented!("Invalid predicate for the json path DSL: {predicate}"),
        }
    }
}

impl JsonPathDsl<Value> for RegexWrapper<String> {
    fn eval(
        &self,
        jsonpath_res: JsonPathResult<'_, Value>,
        predicate: Predicate,
        log_settings: &LogSettings,
    ) -> Assertion<Value> {
        match predicate {
            Matches => self.matches(jsonpath_res).assert(log_settings),
            DoesNotMatch => self.does_not_match(jsonpath_res).assert(log_settings),
            _ => unimplemented!("[TEST] Invalid predicate for the json path DSL: {predicate}"),
        }
    }
}

impl JsonPathDsl<Value> for RegexWrapper<&str> {
    fn eval(
        &self,
        jsonpath_res: JsonPathResult<'_, Value>,
        predicate: Predicate,
        log_settings: &LogSettings,
    ) -> Assertion<Value> {
        match predicate {
            Matches => self.matches(jsonpath_res).assert(log_settings),
            DoesNotMatch => self.does_not_match(jsonpath_res).assert(log_settings),
            _ => unimplemented!("Invalid predicate for the json path DSL: {predicate}"),
        }
    }
}

/// Http json body DSL to assert body of a response.
pub trait JsonPathValueDsl<T>: JsonPathDsl<T> {
    /// Asserts that the json path value is strictly equals to the provided value.
    fn is(&self, jsonpath_res: JsonPathResult<'_, T>) -> Assertion<Value>;
    /// Asserts that the json path value is strictly not equals to the provided value.
    fn is_not(&self, jsonpath_res: JsonPathResult<'_, T>) -> Assertion<Value>;
    /// Asserts that the value of the json path matches the json schema.
    fn schema(&self, jsonpath_res: JsonPathResult<'_, T>) -> Assertion<Value>;
    /// Asserts that the json path value contains the provided value.
    fn contains(&self, jsonpath_res: JsonPathResult<'_, T>) -> Assertion<Value>;
    /// Asserts that the json path value does not contain the provided value.
    fn does_not_contain(&self, jsonpath_res: JsonPathResult<'_, T>) -> Assertion<Value>;
}

/// Http json path regex DSL.
pub trait JsonPathRegexDsl<T>: JsonPathDsl<T> {
    /// Asserts that the json path value matches the regex.
    fn matches(&self, jsonpath_res: JsonPathResult<'_, T>) -> Assertion<Value>;
    /// Asserts that the json path value does not match the regex.
    fn does_not_match(&self, jsonpath_res: JsonPathResult<'_, T>) -> Assertion<Value>;
    /// Evaluates the json body assertion to run based on the [`Predicate`].
    fn eval(
        &self,
        jsonpath_res: JsonPathResult<'_, T>,
        predicate: Predicate,
        log_settings: &LogSettings,
    ) -> Assertion<Value> {
        match predicate {
            Matches => self.matches(jsonpath_res).assert(log_settings),
            DoesNotMatch => self.does_not_match(jsonpath_res).assert(log_settings),
            _ => unimplemented!("[TEST] Invalid predicate for the json path DSL: {predicate}"),
        }
    }
}

impl JsonPathValueDsl<Value> for Value {
    fn is(&self, jsonpath_res: JsonPathResult<'_, Value>) -> Assertion<Value> {
        jsonpath_res.is_eq(self)
    }

    fn is_not(&self, jsonpath_res: JsonPathResult<'_, Value>) -> Assertion<Value> {
        jsonpath_res.is_ne(self)
    }

    fn schema(&self, jsonpath_res: JsonPathResult<'_, Value>) -> Assertion<Value> {
        jsonpath_res.matches_schema(self)
    }

    fn contains(&self, jsonpath_res: JsonPathResult<'_, Value>) -> Assertion<Value> {
        jsonpath_res.has(self)
    }

    fn does_not_contain(&self, jsonpath_res: JsonPathResult<'_, Value>) -> Assertion<Value> {
        jsonpath_res.has_not(self)
    }
}

impl JsonPathValueDsl<Value> for String {
    fn is(&self, jsonpath_res: JsonPathResult<'_, Value>) -> Assertion<Value> {
        jsonpath_res.is_eq(self)
    }

    fn is_not(&self, jsonpath_res: JsonPathResult<'_, Value>) -> Assertion<Value> {
        jsonpath_res.is_ne(self)
    }

    fn schema(&self, jsonpath_res: JsonPathResult<'_, Value>) -> Assertion<Value> {
        jsonpath_res.matches_schema(self)
    }

    fn contains(&self, jsonpath_res: JsonPathResult<'_, Value>) -> Assertion<Value> {
        jsonpath_res.has(self)
    }

    fn does_not_contain(&self, jsonpath_res: JsonPathResult<'_, Value>) -> Assertion<Value> {
        jsonpath_res.has_not(self)
    }
}

impl JsonPathValueDsl<Value> for &str {
    fn is(&self, jsonpath_res: JsonPathResult<'_, Value>) -> Assertion<Value> {
        jsonpath_res.is_eq(*self)
    }

    fn is_not(&self, jsonpath_res: JsonPathResult<'_, Value>) -> Assertion<Value> {
        jsonpath_res.is_ne(*self)
    }

    fn schema(&self, jsonpath_res: JsonPathResult<'_, Value>) -> Assertion<Value> {
        jsonpath_res.matches_schema(*self)
    }

    fn contains(&self, jsonpath_res: JsonPathResult<'_, Value>) -> Assertion<Value> {
        jsonpath_res.has(*self)
    }

    fn does_not_contain(&self, jsonpath_res: JsonPathResult<'_, Value>) -> Assertion<Value> {
        jsonpath_res.has_not(*self)
    }
}

impl JsonPathValueDsl<Value> for PathBuf {
    fn is(&self, jsonpath_res: JsonPathResult<'_, Value>) -> Assertion<Value> {
        jsonpath_res.is_eq(self)
    }

    fn is_not(&self, jsonpath_res: JsonPathResult<'_, Value>) -> Assertion<Value> {
        jsonpath_res.is_ne(self)
    }

    fn schema(&self, jsonpath_res: JsonPathResult<'_, Value>) -> Assertion<Value> {
        jsonpath_res.matches_schema(self)
    }

    fn contains(&self, jsonpath_res: JsonPathResult<'_, Value>) -> Assertion<Value> {
        jsonpath_res.has(self)
    }

    fn does_not_contain(&self, jsonpath_res: JsonPathResult<'_, Value>) -> Assertion<Value> {
        jsonpath_res.has_not(self)
    }
}

impl JsonPathRegexDsl<Value> for RegexWrapper<String> {
    fn matches(&self, jsonpath_res: JsonPathResult<'_, Value>) -> Assertion<Value> {
        jsonpath_res.is_match(&self.0)
    }

    fn does_not_match(&self, jsonpath_res: JsonPathResult<'_, Value>) -> Assertion<Value> {
        jsonpath_res.is_not_match(&self.0)
    }
}

impl JsonPathRegexDsl<Value> for RegexWrapper<&str> {
    fn matches(&self, jsonpath_res: JsonPathResult<'_, Value>) -> Assertion<Value> {
        jsonpath_res.is_match(self.0)
    }

    fn does_not_match(&self, jsonpath_res: JsonPathResult<'_, Value>) -> Assertion<Value> {
        jsonpath_res.is_not_match(self.0)
    }
}
