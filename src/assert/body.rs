//! A collection of body matchers to use with [`Assert::body()`].
//!
//! [`Assert::body()`]: crate::Assert::body
#[cfg(feature = "diff")]
use pretty_assertions::assert_eq;
use serde_json::Value;

/// A generic body exact matcher.
pub trait BodyExactMatcher {
    /// A function to define how the json body will be evaluated
    /// compared to the provided one.
    ///
    /// It is recommended to use assertions to take advantage of
    /// the debug messages and the pretty assertions when
    /// the `diff` feature is enabled.
    fn matches(&self, other: Option<&Value>);
}

impl BodyExactMatcher for String {
    fn matches(&self, actual: Option<&Value>) {
        let expected: Option<Value> = serde_json::from_str(self).ok();
        assert_eq!(
            expected.as_ref(),
            actual,
            "The json body doesn't match the expected one. Expected : {:#?}, Found : {:#?}",
            expected,
            actual,
        );
    }
}

impl BodyExactMatcher for &str {
    fn matches(&self, actual: Option<&Value>) {
        let expected: Option<Value> = serde_json::from_str(self).ok();
        assert_eq!(
            expected.as_ref(),
            actual,
            "The json body doesn't match the expected one. Expected : {:#?}, Found : {:#?}",
            expected,
            actual,
        );
    }
}

impl BodyExactMatcher for Value {
    fn matches(&self, actual: Option<&Value>) {
        let expected = Some(self);

        assert_eq!(
            expected, actual,
            "The json body doesn't match the expected one. Expected : {:#?}, Found : {:#?}",
            expected, actual,
        );
    }
}
