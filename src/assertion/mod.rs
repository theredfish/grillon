mod equality;
mod range;

use crate::dsl::{Part, Predicate};
use std::fmt::Debug;

pub enum Hand<T>
where
    T: Debug,
{
    Left(T),
    Right(T),
    Range(T, T),
}

pub struct Assertion<T>
where
    T: Debug,
{
    pub part: Part,
    pub predicate: Predicate,
    pub left: Hand<T>,
    pub right: Hand<T>,
    pub result: AssertionResult,
}

pub enum AssertionResult {
    Passed,
    Failed,
    NotYetStarted,
}

impl<T> Assertion<T>
where
    T: Debug,
{
    pub fn passed(self) -> bool {
        match self.result {
            AssertionResult::Passed => true,
            AssertionResult::Failed | AssertionResult::NotYetStarted => false,
        }
    }

    pub fn failed(self) -> bool {
        match self.result {
            AssertionResult::Failed => true,
            AssertionResult::Passed | AssertionResult::NotYetStarted => false,
        }
    }

    fn message(&self) -> String {
        let result = &self.result;

        let predicate = &self.predicate;
        let part = &self.part;
        let left = match &self.left {
            Hand::Left(left) => format!("{left:#?}"),
            Hand::Range(min, max) => format!("{min:#?} and {max:#?}"),
            Hand::Right(_) => "".to_string(),
        };
        let right = match &self.right {
            Hand::Right(right) => format!("{right:#?}"),
            Hand::Range(min, max) => format!("{min:#?} and {max:#?}"),
            Hand::Left(_) => "".to_string(),
        };

        // The base message is built as a passing case.
        let message = match part {
            Part::Empty => format!("{left} {predicate} {right}"),
            _ => format!("{part} {predicate} {right}"),
        };

        match result {
            AssertionResult::Passed => message,
            AssertionResult::Failed => format!("{message}. Found {left}"),
            AssertionResult::NotYetStarted => format!("Not yet started : {message}"),
        }
    }
}

impl From<bool> for AssertionResult {
    fn from(val: bool) -> Self {
        if val {
            AssertionResult::Passed
        } else {
            AssertionResult::Failed
        }
    }
}
