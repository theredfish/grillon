use crate::{
    assert::Assertion,
    dsl::{expression::Operator, NumberComparand},
};

pub trait TimeDsl<T> {
    fn is_between(&self, actual: &T) -> Assertion;
    fn is_less_than(&self, actual: &T) -> Assertion;
    fn eval(&self, actual: &T, operator: Operator) -> Assertion {
        match operator {
            Operator::IsLessThan => self.is_less_than(&actual),
            Operator::IsBetween => self.is_between(&actual),
            _ => unreachable!(),
        }
    }
}

impl TimeDsl<u128> for NumberComparand<u128> {
    fn is_less_than(&self, actual: &u128) -> Assertion {
        let expected = &self.left;
        let result = actual < expected;
        let message = format!("Response time assertion failed: ({actual}ms) is over {expected}ms");

        Assertion { result, message }
    }

    fn is_between(&self, actual: &u128) -> Assertion {
        // `self.right` is constrained by the expression signature and can never be `None`
        let (min, max) = (self.left, self.right.unwrap());
        let result = actual >= &min && actual <= &max;
        let message = format!(
            "Response time assertion failed: ({actual}ms) should be between {min}ms and {max}ms"
        );

        Assertion { result, message }
    }
}
