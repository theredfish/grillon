use crate::{
    assertion::{traits::LessThan, Assertion},
    dsl::expression::Predicate,
    LogSettings,
};

pub trait JsonPathDsl<T> {
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
