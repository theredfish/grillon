use crate::assert::{
    Assertion,
    AssertionType::{Equals, NotEquals},
};
use crate::dsl::{
    expression::Predicate::{self, Is, IsNot},
    part::Part,
};
use serde_json::Value;

/// Http json body DSL to assert body of a response.
pub trait JsonBodyDsl<T> {
    /// Asserts the json response body is strictly equals to the provided value.
    fn is(&self, actual: &T);
    /// Asserts the json response body is strictly not equals to the provided value.
    fn is_not(&self, actual: &T);
    /// Evaluates the json body assertion to run based on the [`Predicate`].
    fn eval(&self, actual: &T, predicate: Predicate) {
        match predicate {
            Is => self.is(actual),
            IsNot => self.is_not(actual),
            _ => unimplemented!("Invalid predicate for the json body DSL : {predicate}"),
        }
    }
}

impl JsonBodyDsl<Value> for Value {
    fn is(&self, actual: &Value) {
        Assertion::emit(actual, self, Equals, Is, Part::JsonBody)
    }

    fn is_not(&self, actual: &Value) {
        Assertion::emit(actual, self, NotEquals, IsNot, Part::JsonBody)
    }
}

impl JsonBodyDsl<Value> for &str {
    fn is(&self, actual: &Value) {
        let expected: Value = serde_json::from_str(self).ok().unwrap();
        Assertion::emit(actual, &expected, Equals, Is, Part::JsonBody)
    }

    fn is_not(&self, actual: &Value) {
        let expected: Value = serde_json::from_str(self).ok().unwrap();
        Assertion::emit(actual, &expected, NotEquals, IsNot, Part::JsonBody)
    }
}

impl JsonBodyDsl<Value> for String {
    fn is(&self, actual: &Value) {
        let expected: Value = serde_json::from_str(self).ok().unwrap();
        Assertion::emit(actual, &expected, Equals, Is, Part::JsonBody)
    }

    fn is_not(&self, actual: &Value) {
        let expected: Value = serde_json::from_str(self).ok().unwrap();
        Assertion::emit(actual, &expected, NotEquals, IsNot, Part::JsonBody)
    }
}
