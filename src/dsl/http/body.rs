use crate::{assert::Assertion, dsl::expression::Operator};
use serde_json::Value;

pub trait JsonBodyDsl<T> {
    fn is(&self, actual: &T) -> Assertion;
    fn is_not(&self, actual: &T) -> Assertion;
    fn eval(&self, actual: &T, operator: Operator) -> Assertion {
        match operator {
            Operator::Is => self.is(actual),
            Operator::IsNot => self.is_not(actual),
            _ => unimplemented!(),
        }
    }
}

impl JsonBodyDsl<Value> for Value {
    fn is(&self, actual: &Value) -> Assertion {
        let result = self == actual;
        let message = "The json body doesn't exactly match the expected one".to_string();

        Assertion { result, message }
    }

    fn is_not(&self, actual: &Value) -> Assertion {
        unimplemented!()
    }
}

impl JsonBodyDsl<Value> for &str {
    fn is(&self, actual: &Value) -> Assertion {
        let expected: Value = serde_json::from_str(self).ok().unwrap();
        let result = &expected == actual;
        let message = format!(
            "The json body doesn't exactly match the expected one. actual = {}, expected = {:?}",
            actual, self
        );

        Assertion { result, message }
    }

    fn is_not(&self, actual: &Value) -> Assertion {
        unimplemented!()
    }
}

impl JsonBodyDsl<Value> for String {
    fn is(&self, actual: &Value) -> Assertion {
        let expected: Value = serde_json::from_str(self).ok().unwrap();
        let result = &expected == actual;
        let message = format!(
            "The json body doesn't exactly match the expected one. actual = {}, expected = {:?}",
            actual, self
        );

        Assertion { result, message }
    }

    fn is_not(&self, actual: &Value) -> Assertion {
        unimplemented!()
    }
}
