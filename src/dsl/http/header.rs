use crate::{assert::Assertion, dsl::expression::Operator, header::HeaderMap};

pub trait HeaderDsl<T> {
    fn is(&self, actual: &T) -> Assertion;
    fn eval(&self, actual: &T, operator: Operator) -> Assertion {
        match operator {
            Operator::Is => self.is(actual),
            _ => unreachable!(),
        }
    }
}

impl HeaderDsl<HeaderMap> for HeaderMap {
    fn is(&self, actual: &HeaderMap) -> Assertion {
        let result = self == actual;
        let message = format!(
            "Headers assertion failed : should be {:?}, found: {:?}",
            actual, self
        );

        Assertion { result, message }
    }
}
