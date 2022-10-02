use serde::{Deserialize, Serialize};
use strum::Display;

/// Type representing a condition for assertions.
///
/// [`Predicate`]s are used in the various DSL modules to apply conditions
/// in assertions in a declarative way. A [`Predicate`] is used via an
/// [`Expression`].
#[derive(Display, Serialize, Deserialize, Debug, PartialEq, Eq)]
pub enum Predicate {
    /// Actual should be equals (strictly) to expected.
    #[strum(serialize = "should be")]
    #[serde(rename = "should be")]
    Is,
    /// Actual should not be equals (strictly) to expected.
    #[strum(serialize = "should not be")]
    #[serde(rename = "should not be")]
    IsNot,
    /// Actual should contain expected.
    #[strum(serialize = "should contain")]
    #[serde(rename = "should contain")]
    Contains,
    /// Actual should not contain expected.
    #[strum(serialize = "should not contain")]
    #[serde(rename = "should not contain")]
    DoesNotContain,
    /// Actual should match the regex.
    #[strum(serialize = "should match")]
    #[serde(rename = "should match")]
    Matches,
    /// Actual should not match the regex.
    #[strum(serialize = "should not match")]
    #[serde(rename = "should not match")]
    DoesNotMatch,
    /// Actual should be less than expected.
    #[strum(serialize = "should be less than")]
    #[serde(rename = "should be less than")]
    LessThan,
    /// Actual should match the json path.
    #[strum(serialize = "should match json path")]
    #[serde(rename = "should match json path")]
    JsonPath,
    /// Actual should be between the given closed interval [min, max].
    #[strum(serialize = "should be between")]
    #[serde(rename = "should be between")]
    Between,
}

/// Represents a range starting with `left` and ending with `right`.
///
/// This type does not assume if it is a closed, open or half-closed/open interval.
/// Its use will determine it.
#[derive(Deserialize, Debug, PartialEq, Eq)]
pub struct Range<T> {
    /// The left value of the range.
    pub left: T,
    /// The right value of the range.
    pub right: T,
}

/// Represents an expected `value` associated to a [`Predicate`] to run against
/// another `value`.
///
/// An expression is used to build assertions. It is composed of a [`Predicate`]
/// and an expected `value` that will be used to create expressive assertion
/// functions like this one : `status(is_between(200, 204))`. In this example we
/// assert that the actual [`StatusCode`] is [`Between`] a closed [`Range`].
///
/// [`Between`]: Predicate::Between
/// [`StatusCode`]: crate::StatusCode
#[derive(Deserialize, Debug, PartialEq, Eq)]
pub struct Expression<T> {
    /// The [`Predicate`] to apply in an assertion.
    pub predicate: Predicate,
    /// The expected value as part of the [`Predicate`].
    pub value: T,
}

/// Macro to generate assertion functions that return an [`Expression`].
macro_rules! predicate {
    ($(#[$meta:meta])* $name:ident, $o:expr) => {
        $(#[$meta])*
        pub fn $name<T>(value: T) -> Expression<T> {
            Expression {
                predicate: $o,
                value,
            }
        }
    };
}

/// Creates an expression to assert the actual value is in the closed interval [min, max].
pub fn is_between<T>(min: T, max: T) -> Expression<Range<T>> {
    Expression {
        predicate: Predicate::Between,
        value: Range {
            left: min,
            right: max,
        },
    }
}

predicate!(
    /// Creates an expression to assert that the actual value is strictly equal to the expected one.
    is,
    Predicate::Is
);
predicate!(
    /// Creates an expression to assert that the actual value is strictly not equal to the expected one.
    is_not,
    Predicate::IsNot
);
predicate!(
    /// Creates an expression to assert that the actual value contains the expected one.
    contains,
    Predicate::Contains
);
predicate!(
    /// Creates an expression to assert that the actual value does not contain the expected one.
    does_not_contain,
    Predicate::DoesNotContain
);
predicate!(
    /// Creates an expression to assert that the actual value matches the regex.
    matches,
    Predicate::Matches
);
predicate!(
    /// Creates an expression to assert that the actual value does not match the regex.
    does_not_match,
    Predicate::DoesNotMatch
);
predicate!(
    /// Creates an expression to assert that the actual value matches the json path.
    jsonpath,
    Predicate::JsonPath
);
predicate!(
    /// Creates an expression to assert that the actual value is inferior to the provided value.
    is_less_than,
    Predicate::LessThan
);

#[cfg(test)]
pub mod tests {
    use super::{Expression, Predicate, Range};
    use serde_json::Value;
    use test_case::test_case;

    #[test_case(Value::String(String::from("should be")), Predicate::Is; "Failed to deserialize predicate Is")]
    #[test_case(Value::String(String::from("should not be")), Predicate::IsNot; "Failed to deserialize predicate IsNot")]
    #[test_case(Value::String(String::from("should contain")), Predicate::Contains; "Failed to deserialize predicate Contains")]
    #[test_case(Value::String(String::from("should not contain")), Predicate::DoesNotContain; "Failed to deserialize predicate DoesNotContain")]
    #[test_case(Value::String(String::from("should match")), Predicate::Matches; "Failed to deserialize predicate Matches")]
    #[test_case(Value::String(String::from("should not match")), Predicate::DoesNotMatch; "Failed to deserialize predicate DoesNotMatch")]
    #[test_case(Value::String(String::from("should match json path")), Predicate::JsonPath; "Failed to deserialize predicate JsonPath")]
    #[test_case(Value::String(String::from("should be less than")), Predicate::LessThan; "Failed to deserialize predicate LessThan")]
    #[test_case(Value::String(String::from("should be between")), Predicate::Between; "Failed to deserialize predicate Between")]

    fn deser_predicates(json_predicate: Value, predicate: Predicate) {
        assert_eq!(
            serde_json::from_value::<Predicate>(json_predicate).unwrap(),
            predicate
        )
    }

    #[test]
    fn deser_expression() {
        let json = serde_json::json!({
            "predicate": "should be between",
            "value": {
                "left": 200,
                "right": 299
            }
        });

        let expr: Expression<Range<u16>> = serde_json::from_value(json).unwrap();

        assert_eq!(
            expr,
            Expression {
                predicate: Predicate::Between,
                value: Range {
                    left: 200,
                    right: 299
                }
            }
        );
    }
}
