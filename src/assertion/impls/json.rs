use crate::{
    assertion::{traits::Equality, Assertion, Hand},
    dsl::{Part, Predicate},
};
use serde_json::Value;

impl Equality<Value> for Value {
    type Assertion = Assertion<Value>;

    fn is_eq(&self, rhs: &Value) -> Self::Assertion {
        let result = self == rhs;

        Assertion {
            predicate: Predicate::Is,
            part: Part::JsonBody,
            left: Hand::Left(self.clone()),
            right: Hand::Right(rhs.clone()),
            result: result.into(),
        }
    }

    fn is_ne(&self, rhs: &Value) -> Self::Assertion {
        let result = self != rhs;

        Assertion {
            predicate: Predicate::IsNot,
            part: Part::JsonBody,
            left: Hand::Left(self.clone()),
            right: Hand::Right(rhs.clone()),
            result: result.into(),
        }
    }
}

impl Equality<str> for Value {
    type Assertion = Assertion<Value>;

    fn is_eq(&self, rhs: &str) -> Self::Assertion {
        let rhs = serde_json::from_str(rhs).unwrap();
        let result = self == &rhs;

        Assertion {
            predicate: Predicate::Is,
            part: Part::JsonBody,
            left: Hand::Left(self.clone()),
            right: Hand::Right(rhs),
            result: result.into(),
        }
    }

    fn is_ne(&self, rhs: &str) -> Self::Assertion {
        let rhs = serde_json::from_str(rhs).unwrap();
        let result = self != &rhs;

        Assertion {
            predicate: Predicate::IsNot,
            part: Part::JsonBody,
            left: Hand::Left(self.clone()),
            right: Hand::Right(rhs),
            result: result.into(),
        }
    }
}

impl Equality<String> for Value {
    type Assertion = Assertion<Value>;

    fn is_eq(&self, rhs: &String) -> Self::Assertion {
        let rhs = serde_json::from_str(rhs).unwrap();
        let result = self == &rhs;

        Assertion {
            predicate: Predicate::Is,
            part: Part::JsonBody,
            left: Hand::Left(self.clone()),
            right: Hand::Right(rhs),
            result: result.into(),
        }
    }

    fn is_ne(&self, rhs: &String) -> Self::Assertion {
        let rhs = serde_json::from_str(rhs).unwrap();
        let result = self != &rhs;

        Assertion {
            predicate: Predicate::IsNot,
            part: Part::JsonBody,
            left: Hand::Left(self.clone()),
            right: Hand::Right(rhs),
            result: result.into(),
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::assertion::traits::Equality;
    use serde_json::{json, Value};

    fn value_stub() -> Value {
        json!({
            "a_string": "john",
            "a_number": 12,
            "a_string_number": "12",
            "a_vec": [
                {
                    "entry1": "entry1"
                },
                {
                    "entry2": "entry2"
                }
            ]
        })
    }

    #[test]
    fn impl_is_eq_value() {
        let assertion = value_stub().is_eq(&value_stub());
        assert!(assertion.passed(), "{}", assertion.log());
    }

    #[test]
    fn impl_is_eq_with_different_order() {
        let json_value = json!({
            "a_string_number": "12",
            "a_number": 12,
            "a_vec": [
                {
                    "entry1": "entry1"
                },
                {
                    "entry2": "entry2"
                }
                ],
            "a_string": "john",
        });
        let json_string = json_value.to_string();
        let json_str = json_string.as_str();

        let value_assertion = value_stub().is_eq(&json_value);
        let str_assertion = value_stub().is_eq(json_str);
        let string_assertion = value_stub().is_eq(&json_string);

        assert!(value_assertion.passed(), "{}", value_assertion.log());
        assert!(str_assertion.passed(), "{}", str_assertion.log());
        assert!(string_assertion.passed(), "{}", string_assertion.log());
    }

    #[test]
    fn impl_is_eq_str() {
        let json_str = r#"{
            "a_string": "john",
            "a_number": 12,
            "a_string_number": "12",
            "a_vec": [
                {
                    "entry1": "entry1"
                },
                {
                    "entry2": "entry2"
                }
            ]
        }"#;

        let assertion = value_stub().is_eq(json_str);
        assert!(assertion.passed(), "{}", assertion.log());
    }

    #[test]
    fn impl_is_eq_string() {
        let json_string = r#"{
            "a_string": "john",
            "a_number": 12,
            "a_string_number": "12",
            "a_vec": [
                {
                    "entry1": "entry1"
                },
                {
                    "entry2": "entry2"
                }
            ]
        }"#
        .to_string();

        let assertion = value_stub().is_eq(&json_string);
        assert!(assertion.passed(), "{}", assertion.log());
    }

    #[test]
    fn impl_is_ne_value() {
        let other_value = json!({
            "another_string": "john"
        });
        let assertion = value_stub().is_ne(&other_value);
        assert!(assertion.passed(), "{}", assertion.log());
    }

    #[test]
    fn impl_is_ne_str() {
        let json_str = r#"{
            "a_string": "john",
            "a_number": 12
        }"#;

        let assertion = value_stub().is_ne(json_str);
        assert!(assertion.passed(), "{}", assertion.log());
    }

    #[test]
    fn impl_is_ne_string() {
        let json_string = r#"{
            "a_string": "john",
            "a_number": 12
        }"#
        .to_string();

        let assertion = value_stub().is_ne(&json_string);
        assert!(assertion.passed(), "{}", assertion.log());
    }

    #[test]
    fn impl_is_ne_different_type() {
        let assertion = json!({"age": "12"}).is_ne(r#"{"age": 12}"#);
        assert!(assertion.passed(), "{}", assertion.log());
    }

    mod serialization {
        use super::*;
        use serde_json::json;

        #[test]
        fn it_serializes_json_body_should_be() {
            let response_payload = json!({
                "user": "john",
                "age": 23
            });

            let expected_json = json!({
                "part": "json body",
                "predicate": "should be",
                "left": response_payload,
                "right": response_payload,
                "result": "passed"
            });

            let assertion = response_payload.is_eq(&response_payload);

            assert_eq!(
                json!(assertion),
                expected_json,
                "Serialized assertion is not equals to the expected json",
            );
        }

        #[test]
        fn it_serializes_json_body_should_not_be() {
            let response_payload = json!({
                "user": "john",
                "age": 23
            });

            let expected_json = json!({
                "part": "json body",
                "predicate": "should not be",
                "left": value_stub(),
                "right": response_payload,
                "result": "passed"
            });

            let assertion = value_stub().is_ne(&response_payload);

            assert_eq!(
                json!(assertion),
                expected_json,
                "Serialized assertion is not equals to the expected json",
            );
        }
    }
}
