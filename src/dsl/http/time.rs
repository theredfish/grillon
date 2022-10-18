use crate::{
    assertion::{traits::LessThan, Assertion},
    dsl::expression::Predicate,
    LogSettings,
};

/// Http time DSL to assert the response time.
pub trait TimeDsl<T> {
    /// Asserts the response time is strictly inferior to the provided time in
    /// milliseconds.
    fn is_less_than(&self, actual: T) -> Assertion<u64>;
    /// Evaluates the time assertion to run based on the [`Predicate`]
    fn eval(&self, actual: T, operator: Predicate, log_settings: &LogSettings) -> Assertion<u64> {
        match operator {
            Predicate::LessThan => self.is_less_than(actual).assert(log_settings),
            _ => unimplemented!(),
        }
    }
}

impl TimeDsl<u64> for u64 {
    fn is_less_than(&self, actual: u64) -> Assertion<u64> {
        actual.less_than(self)
    }
}
