use crate::{assert::Assertion, dsl::expression::Operator};
use http::StatusCode;

pub trait StatusCodeDsl<T> {
    fn is(&self, actual: &T) -> Assertion;
    fn is_not(&self, actual: &T) -> Assertion;
    fn eval(&self, actual: &T, operator: Operator) -> Assertion {
        match operator {
            Operator::Is => self.is(actual),
            Operator::IsNot => self.is_not(actual),
            _ => unreachable!(),
        }
    }
}

impl StatusCodeDsl<StatusCode> for u16 {
    fn is(&self, actual: &StatusCode) -> Assertion {
        let result = self == &actual.as_u16();
        let message = format!("Status assertion failed: should be {actual}, found {self}");

        Assertion { result, message }
    }

    fn is_not(&self, actual: &StatusCode) -> Assertion {
        let result = self != &actual.as_u16();
        let message = format!("Status assertion failed: should be different from {self}");

        Assertion { result, message }
    }
}

impl StatusCodeDsl<StatusCode> for StatusCode {
    fn is(&self, actual: &StatusCode) -> Assertion {
        let result = self == actual;
        let actual = actual.as_u16();
        let expected = self.as_u16();

        let message = format!("Status assertion failed: should be {actual}, found {expected}");
        Assertion { result, message }
    }

    fn is_not(&self, actual: &StatusCode) -> Assertion {
        let result = self != actual;
        let expected = self.as_u16();
        let message = format!("Status assertion failed: should be different from {expected}");

        Assertion { result, message }
    }
}
