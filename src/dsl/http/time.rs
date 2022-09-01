use crate::{
    assert::{Assertion, AssertionType::Test},
    dsl::{
        expression::Predicate::{self, LessThan},
        part::Part,
    },
};

/// Http time DSL to assert the response time.
pub trait TimeDsl<T> {
    /// Asserts the response time is strictly inferior to the provided time in
    /// milliseconds.
    fn is_less_than(&self, actual: &T);
    /// Evaluates the time assertion to run based on the [`Predicate`]
    fn eval(&self, actual: &T, operator: Predicate) {
        match operator {
            Predicate::LessThan => self.is_less_than(actual),
            _ => unimplemented!(),
        }
    }
}

impl TimeDsl<u128> for u128 {
    fn is_less_than(&self, actual: &u128) {
        let result = actual < self;

        Assertion::emit(actual, self, Test(result), LessThan, Part::ResponseTime)
    }
}
