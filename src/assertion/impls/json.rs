use crate::{
    assertion::{
        traits::{Equality, JsonSchema},
        Assertion, AssertionResult, Hand, UnprocessableReason,
    },
    dsl::{Part, Predicate},
};
use jsonschema::{output::BasicOutput, JSONSchema};
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

impl JsonSchema<Value> for Value {
    type Assertion = Assertion<Value>;

    fn matches_schema(&self, schema: &Value) -> Self::Assertion {
        let schema = match JSONSchema::compile(schema) {
            Ok(schema) => schema,
            Err(err) => {
                return Assertion {
                    predicate: Predicate::Schema,
                    part: Part::JsonBody,
                    left: Hand::Left(self.clone()),
                    right: Hand::Empty,
                    result: AssertionResult::Unprocessable(UnprocessableReason::InvalidJsonSchema(
                        err.schema_path,
                        err.instance_path,
                    )),
                }
            }
        };

        // Get the boolean result of the validation
        let result = schema.is_valid(self);

        // Generate a json output of the json schema result
        let output: BasicOutput<'_> = schema.apply(self).basic();
        let output = match serde_json::to_value(output) {
            Ok(json_output) => json_output,
            Err(_) => {
                return Assertion {
                    predicate: Predicate::Schema,
                    part: Part::JsonBody,
                    left: Hand::Empty,
                    right: Hand::Empty,
                    result: AssertionResult::Unprocessable(UnprocessableReason::Other(
                        "Failed to serialize json schema error".to_string(),
                    )),
                }
            }
        };

        Assertion {
            predicate: Predicate::Schema,
            part: Part::JsonBody,
            left: Hand::Left(self.clone()),
            right: Hand::Right(output),
            result: result.into(),
        }
    }
}

impl JsonSchema<str> for Value {
    type Assertion = Assertion<Value>;

    fn matches_schema(&self, schema: &str) -> Self::Assertion {
        let schema = match serde_json::from_str(schema) {
            Ok(schema) => schema,
            Err(err) => {
                return Assertion {
                    predicate: Predicate::Schema,
                    part: Part::JsonBody,
                    left: Hand::Empty,
                    right: Hand::Empty,
                    result: AssertionResult::Unprocessable(
                        UnprocessableReason::SerializationFailure(err.to_string()),
                    ),
                }
            }
        };

        let schema = match JSONSchema::compile(&schema) {
            Ok(schema) => schema,
            Err(err) => {
                return Assertion {
                    predicate: Predicate::Schema,
                    part: Part::JsonBody,
                    left: Hand::Left(self.clone()),
                    right: Hand::Empty,
                    result: AssertionResult::Unprocessable(UnprocessableReason::InvalidJsonSchema(
                        err.schema_path,
                        err.instance_path,
                    )),
                }
            }
        };

        // Get the boolean result of the validation
        let result = schema.is_valid(self);

        // Generate a json output of the json schema result
        let output: BasicOutput<'_> = schema.apply(self).basic();
        let output = match serde_json::to_value(output) {
            Ok(json_output) => json_output,
            Err(_) => {
                return Assertion {
                    predicate: Predicate::Schema,
                    part: Part::JsonBody,
                    left: Hand::Empty,
                    right: Hand::Empty,
                    result: AssertionResult::Unprocessable(UnprocessableReason::Other(
                        "Failed to serialize json schema error".to_string(),
                    )),
                }
            }
        };

        Assertion {
            predicate: Predicate::Schema,
            part: Part::JsonBody,
            left: Hand::Left(self.clone()),
            right: Hand::Right(output),
            result: result.into(),
        }
    }
}

impl JsonSchema<String> for Value {
    type Assertion = Assertion<Value>;

    fn matches_schema(&self, schema: &String) -> Self::Assertion {
        let schema = match serde_json::from_str(schema) {
            Ok(schema) => schema,
            Err(_) => {
                return Assertion {
                    predicate: Predicate::Schema,
                    part: Part::JsonBody,
                    left: Hand::Empty,
                    right: Hand::Empty,
                    result: AssertionResult::Unprocessable(UnprocessableReason::Other(
                        "Failed to serialize json schema".to_string(),
                    )),
                }
            }
        };

        let schema = match JSONSchema::compile(&schema) {
            Ok(schema) => schema,
            Err(err) => {
                return Assertion {
                    predicate: Predicate::Schema,
                    part: Part::JsonBody,
                    left: Hand::Left(self.clone()),
                    right: Hand::Empty,
                    result: AssertionResult::Unprocessable(UnprocessableReason::InvalidJsonSchema(
                        err.schema_path,
                        err.instance_path,
                    )),
                }
            }
        };

        // Get the boolean result of the validation
        let result = schema.is_valid(self);

        // Generate a json output of the json schema result
        let output: BasicOutput<'_> = schema.apply(self).basic();
        let output = match serde_json::to_value(output) {
            Ok(json_output) => json_output,
            Err(_) => {
                return Assertion {
                    predicate: Predicate::Schema,
                    part: Part::JsonBody,
                    left: Hand::Empty,
                    right: Hand::Empty,
                    result: AssertionResult::Unprocessable(UnprocessableReason::Other(
                        "Failed to serialize json schema error".to_string(),
                    )),
                }
            }
        };

        Assertion {
            predicate: Predicate::Schema,
            part: Part::JsonBody,
            left: Hand::Left(self.clone()),
            right: Hand::Right(output),
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

    mod schema {
        use crate::assertion::traits::JsonSchema;
        use serde_json::json;

        #[test]
        fn impl_json_schema_is_valid() {
            let schema = json!({
              "$schema": "http://json-schema.org/draft-04/schema#",
              "title": "Age validation schema",
              "type": "object",
              "properties": {
                "age": {
                  "description": "Age in years",
                  "type": "string",
                  "minimum": 0
                }
              },
              "required": ["age"]
            });

            let assertion = json!({"age": "12"}).matches_schema(&schema);
            assert!(assertion.passed(), "{}", assertion.log());
        }

        #[test]
        fn impl_str_schema_is_valid() {
            let schema = r#"{
              "$schema": "http://json-schema.org/draft-04/schema#",
              "title": "Age validation schema",
              "type": "object",
              "properties": {
                "age": {
                  "description": "Age in years",
                  "type": "string",
                  "minimum": 0
                }
              },
              "required": ["age"]
            }"#;

            let assertion = json!({"age": "12"}).matches_schema(schema);
            assert!(assertion.passed(), "{}", assertion.log());
        }

        #[test]
        fn impl_string_schema_is_valid() {
            let schema = r#"{
              "$schema": "http://json-schema.org/draft-04/schema#",
              "title": "Age validation schema",
              "type": "object",
              "properties": {
                "age": {
                  "description": "Age in years",
                  "type": "string",
                  "minimum": 0
                }
              },
              "required": ["age"]
            }"#
            .to_string();

            let assertion = json!({"age": "12"}).matches_schema(&schema);
            assert!(assertion.passed(), "{}", assertion.log());
        }

        #[test]
        fn impl_schema_is_invalid() {
            let schema: serde_json::Value = json!({
              "$schema": "http://json-schema.org/draft-04/schema#",
              "title": "Age validation schema",
              "type": "object",
              "properties": {
                "age": {
                  "description": "Age in years",
                  "type": "number",
                  "minimum": 0
                }
              },
              "required": ["age"]
            });

            // providing a string instead of a number should fail
            let assertion = json!({"age": "12"}).matches_schema(&schema);
            assert!(assertion.failed(), "{}", assertion.log());
        }

        #[test]
        fn impl_schema_compile_error() {
            let schema: serde_json::Value = json!({
              "$schema": "http://json-schema.org/draft-04/schema#",
              "title": "Bad json schema",
              "type": "object",
              "properties": {
                "age": {
                  "description": "Age in years",
                  "type": "string",
                  "minimum": 0,
                  "required": true // Invalid JSON Schema additional property
                }
              },
              "required": ["age"]
            });

            let assertion = json!({"age": 12}).matches_schema(&schema);
            let log = assertion.log();
            assert!(assertion.failed(), "{log}");
            assert_eq!(log, "Invalid json schema: /properties/properties/additionalProperties/properties/required/type => /properties/age/required");
        }
    }
}
