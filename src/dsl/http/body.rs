use crate::{
    assert::{AssertEq, AssertNe, Assertion},
    dsl::{
        expression::Predicate::{self, Is, IsNot},
        part::Part,
    },
};
use serde_json::Value;

/// Http json body DSL to assert body of a response.
pub trait JsonBodyDsl<T> {
    /// Asserts the json response body is strictly equals to the provided value.
    fn is(&self, actual: T) -> Assertion;
    /// Asserts the json response body is strictly not equals to the provided value.
    fn is_not(&self, actual: T) -> Assertion;
    /// Evaluates the json body assertion to run based on the [`Predicate`].
    fn eval(&self, actual: T, predicate: Predicate) -> Assertion {
        match predicate {
            Is => self.is(actual),
            IsNot => self.is_not(actual),
            _ => unimplemented!("Invalid predicate for the json body DSL : {predicate}"),
        }
    }
}

impl JsonBodyDsl<Value> for Value {
    fn is(&self, actual: Value) -> Assertion {
        let ty = AssertEq {
            left: actual,
            right: self.clone(),
        };

        Assertion::new(Box::new(ty), Is, Part::JsonBody)
    }

    fn is_not(&self, actual: Value) -> Assertion {
        let ty = AssertNe {
            left: actual,
            right: self.clone(),
        };

        Assertion::new(Box::new(ty), IsNot, Part::JsonBody)
    }
}

impl JsonBodyDsl<Value> for &str {
    fn is(&self, actual: Value) -> Assertion {
        let expected: Value = serde_json::from_str(self).ok().unwrap();
        let ty = AssertEq {
            left: actual,
            right: expected,
        };

        Assertion::new(Box::new(ty), Is, Part::JsonBody)
    }

    fn is_not(&self, actual: Value) -> Assertion {
        let expected: Value = serde_json::from_str(self).ok().unwrap();
        let ty = AssertNe {
            left: actual,
            right: expected,
        };

        Assertion::new(Box::new(ty), IsNot, Part::JsonBody)
    }
}

impl JsonBodyDsl<Value> for String {
    fn is(&self, actual: Value) -> Assertion {
        let expected: Value = serde_json::from_str(self).ok().unwrap();
        let ty = AssertEq {
            left: actual,
            right: expected,
        };

        Assertion::new(Box::new(ty), Is, Part::JsonBody)
    }

    fn is_not(&self, actual: Value) -> Assertion {
        let expected: Value = serde_json::from_str(self).ok().unwrap();
        let ty = AssertNe {
            left: actual,
            right: expected,
        };

        Assertion::new(Box::new(ty), IsNot, Part::JsonBody)
    }
}
