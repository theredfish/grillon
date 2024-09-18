use crate::{
    assertion::{
        traits::{Equality, JsonSchema},
        Assertion, AssertionResult, Hand, UnprocessableReason,
    },
    dsl::{Part, Predicate},
};
use jsonschema::{output::BasicOutput, Validator};
use serde_json::Value;
use std::{fs, path::PathBuf};

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
        let rhs: Value = match serde_json::from_str(rhs) {
            Ok(value) => value,
            Err(err) => {
                return Assertion {
                    predicate: Predicate::Is,
                    part: Part::JsonBody,
                    left: Hand::Empty,
                    right: Hand::Empty,
                    result: AssertionResult::Unprocessable(
                        UnprocessableReason::SerializationFailure(err.to_string()),
                    ),
                }
            }
        };
        self.is_eq(&rhs)
    }

    fn is_ne(&self, rhs: &str) -> Self::Assertion {
        let rhs: Value = match serde_json::from_str(rhs) {
            Ok(value) => value,
            Err(err) => {
                return Assertion {
                    predicate: Predicate::IsNot,
                    part: Part::JsonBody,
                    left: Hand::Empty,
                    right: Hand::Empty,
                    result: AssertionResult::Unprocessable(
                        UnprocessableReason::SerializationFailure(err.to_string()),
                    ),
                }
            }
        };
        self.is_ne(&rhs)
    }
}

impl Equality<String> for Value {
    type Assertion = Assertion<Value>;

    fn is_eq(&self, rhs: &String) -> Self::Assertion {
        let rhs: Value = match serde_json::from_str(rhs) {
            Ok(value) => value,
            Err(err) => {
                return Assertion {
                    predicate: Predicate::Is,
                    part: Part::JsonBody,
                    left: Hand::Empty,
                    right: Hand::Empty,
                    result: AssertionResult::Unprocessable(
                        UnprocessableReason::SerializationFailure(err.to_string()),
                    ),
                }
            }
        };
        self.is_eq(&rhs)
    }

    fn is_ne(&self, rhs: &String) -> Self::Assertion {
        let rhs: Value = match serde_json::from_str(rhs) {
            Ok(value) => value,
            Err(err) => {
                return Assertion {
                    predicate: Predicate::IsNot,
                    part: Part::JsonBody,
                    left: Hand::Empty,
                    right: Hand::Empty,
                    result: AssertionResult::Unprocessable(
                        UnprocessableReason::SerializationFailure(err.to_string()),
                    ),
                }
            }
        };
        self.is_ne(&rhs)
    }
}

impl Equality<PathBuf> for Value {
    type Assertion = Assertion<Value>;

    fn is_eq(&self, json_file: &PathBuf) -> Self::Assertion {
        let json_file = match fs::read_to_string(json_file) {
            Ok(content) => content,
            Err(_) => {
                return Assertion {
                    predicate: Predicate::Is,
                    part: Part::JsonBody,
                    left: Hand::Empty,
                    right: Hand::Empty,
                    result: AssertionResult::Unprocessable(UnprocessableReason::Other(format!(
                        "Failed to read json file located at {}",
                        json_file.display()
                    ))),
                }
            }
        };

        let expected_json: Value = match serde_json::from_str(&json_file) {
            Ok(json) => json,
            Err(_) => {
                return Assertion {
                    predicate: Predicate::Is,
                    part: Part::JsonBody,
                    left: Hand::Empty,
                    right: Hand::Empty,
                    result: AssertionResult::Unprocessable(
                        UnprocessableReason::SerializationFailure(
                            "Failed to serialize file content".to_string(),
                        ),
                    ),
                }
            }
        };

        self.is_eq(&expected_json)
    }

    fn is_ne(&self, json_file: &PathBuf) -> Self::Assertion {
        let json_file = match fs::read_to_string(json_file) {
            Ok(content) => content,
            Err(_) => {
                return Assertion {
                    predicate: Predicate::IsNot,
                    part: Part::JsonBody,
                    left: Hand::Empty,
                    right: Hand::Empty,
                    result: AssertionResult::Unprocessable(UnprocessableReason::Other(format!(
                        "Failed to read json file located at {}",
                        json_file.display()
                    ))),
                }
            }
        };

        let expected_json: Value = match serde_json::from_str(&json_file) {
            Ok(json) => json,
            Err(_) => {
                return Assertion {
                    predicate: Predicate::IsNot,
                    part: Part::JsonBody,
                    left: Hand::Empty,
                    right: Hand::Empty,
                    result: AssertionResult::Unprocessable(
                        UnprocessableReason::SerializationFailure(
                            "Failed to serialize file content".to_string(),
                        ),
                    ),
                }
            }
        };

        self.is_ne(&expected_json)
    }
}

impl JsonSchema<Value> for Value {
    type Assertion = Assertion<Value>;

    fn matches_schema(&self, schema: &Value) -> Self::Assertion {
        let schema = match Validator::new(schema) {
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
        let schema: Value = match serde_json::from_str(schema) {
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

        self.matches_schema(&schema)
    }
}

impl JsonSchema<String> for Value {
    type Assertion = Assertion<Value>;

    fn matches_schema(&self, schema: &String) -> Self::Assertion {
        let schema: Value = match serde_json::from_str(schema) {
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

        self.matches_schema(&schema)
    }
}

impl JsonSchema<PathBuf> for Value {
    type Assertion = Assertion<Value>;

    fn matches_schema(&self, schema_file: &PathBuf) -> Self::Assertion {
        let schema_file_content = match fs::read_to_string(schema_file) {
            Ok(content) => content,
            Err(_) => {
                return Assertion {
                    predicate: Predicate::Schema,
                    part: Part::JsonBody,
                    left: Hand::Empty,
                    right: Hand::Empty,
                    result: AssertionResult::Unprocessable(UnprocessableReason::Other(format!(
                        "Failed to read json schema file located at {}",
                        schema_file.display()
                    ))),
                }
            }
        };

        let schema: Value = match serde_json::from_str(&schema_file_content) {
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

        self.matches_schema(&schema)
    }
}

#[cfg(test)]
mod tests {
    use crate::assertion::traits::Equality;
    use serde_json::{json, Value};
    use std::path::PathBuf;

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
    fn impl_is_eq_json_file() {
        let value = value_stub();
        let json_file =
            PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("tests/fixtures/json_body.json");

        let assertion = value.is_eq(&json_file);
        assert!(assertion.passed(), "{}", assertion.log());
    }

    #[test]
    fn impl_is_eq_inexistant_json_file() {
        let value = value_stub();
        let json_file =
            PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("tests/fixtures/inexistant.json");

        let assertion = value.is_eq(&json_file);
        assert!(assertion.failed(), "{}", assertion.log());
        let expected_error_msg_part = "Failed to read json file located at";

        assert!(
            assertion.log().contains(expected_error_msg_part),
            "The error message doesn't contain this: {expected_error_msg_part}"
        );
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

    #[test]
    fn impl_is_ne_json_file() {
        let value = json!({"not_the_same": "content"});
        let json_file =
            PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("tests/fixtures/json_body.json");

        let assertion = value.is_ne(&json_file);
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
        fn impl_schema_validation_error() {
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
