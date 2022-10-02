use crate::{
    assert::{AssertBool, Assertion},
    dsl::{
        expression::Predicate::{self, LessThan},
        part::Part,
    },
};

/// Http time DSL to assert the response time.
pub trait TimeDsl<T> {
    /// Asserts the response time is strictly inferior to the provided time in
    /// milliseconds.
    fn is_less_than(&self, actual: T) -> Assertion;
    /// Evaluates the time assertion to run based on the [`Predicate`]
    fn eval(&self, actual: T, operator: Predicate) -> Assertion {
        match operator {
            Predicate::LessThan => self.is_less_than(actual),
            _ => unimplemented!(),
        }
    }
}

impl TimeDsl<u64> for u64 {
    fn is_less_than(&self, actual: u64) -> Assertion {
        let result = actual < *self;

        let ty = AssertBool {
            left: actual,
            right: *self,
            result,
        };

        Assertion::new(Box::new(ty), LessThan, Part::ResponseTime)
    }
}
