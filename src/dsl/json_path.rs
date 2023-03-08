use crate::{
    assertion::traits::Equality,
    assertion::Assertion,
    dsl::expression::Predicate::{self, Is, IsNot},
    LogSettings,
};
use serde::{Deserialize, Serialize};
use serde_json::Value;

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq)]
pub struct JsonPathResult<'p, T> {
    pub path: &'p str,
    pub value: T,
}

impl<'p, T> JsonPathResult<'p, T> {
    pub fn new(path: &'p str, value: T) -> Self {
        Self { path, value }
    }
}

/// Http json body DSL to assert body of a response.
pub trait JsonPathDsl<T> {
    /// Asserts the json response body is strictly equals to the provided value.
    fn is(&self, jsonpath_res: JsonPathResult<'_, T>) -> Assertion<Value>;
    /// Asserts the json response body is strictly not equals to the provided value.
    fn is_not(&self, jsonpath_res: JsonPathResult<'_, T>) -> Assertion<Value>;
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
            _ => unimplemented!("Invalid predicate for the json body DSL : {predicate}"),
        }
    }
}

impl JsonPathDsl<Value> for Value {
    fn is(&self, jsonpath_res: JsonPathResult<'_, Value>) -> Assertion<Value> {
        // let to_array: Value = self.take();
        match self {
            Value::Array(_) => jsonpath_res.is_eq(&self),
            _ => jsonpath_res.is_eq(&Value::Array(vec![self.clone()])),
        }
    }

    fn is_not(&self, jsonpath_res: JsonPathResult<'_, Value>) -> Assertion<Value> {
        jsonpath_res.is_ne(self)
    }
}
