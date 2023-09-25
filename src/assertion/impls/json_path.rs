use crate::{
    assertion::{
        traits::{Equality, JsonSchema},
        Assertion, AssertionResult, Hand, UnprocessableReason,
    },
    dsl::{json_path::JsonPathResult, Part, Predicate},
};
use jsonschema::{output::BasicOutput, JSONSchema};
use serde_json::Value;
use std::{fs, path::PathBuf};

impl Equality<Value> for JsonPathResult<'_, Value> {
    type Assertion = Assertion<Value>;

    fn is_eq(&self, expected: &Value) -> Self::Assertion {
        let expected = to_value_array(expected);
        let result = self.value == expected;
        Assertion {
            predicate: Predicate::Is,
            part: Part::JsonPath,
            left: Hand::Compound(Value::String(self.path.to_string()), self.value.clone()),
            right: Hand::Right(expected),
            result: result.into(),
        }
    }

    fn is_ne(&self, expected: &Value) -> Self::Assertion {
        let expected = to_value_array(expected);
        let result = self.value != expected;

        Assertion {
            predicate: Predicate::IsNot,
            part: Part::JsonPath,
            left: Hand::Compound(Value::String(self.path.to_string()), self.value.clone()),
            right: Hand::Right(expected),
            result: result.into(),
        }
    }
}

impl Equality<String> for JsonPathResult<'_, Value> {
    type Assertion = Assertion<Value>;

    fn is_eq(&self, expected: &String) -> Self::Assertion {
        let expected: Value = match serde_json::from_str(expected) {
            Ok(value) => value,
            Err(err) => {
                return Assertion {
                    predicate: Predicate::Is,
                    part: Part::JsonPath,
                    left: Hand::Empty,
                    right: Hand::Empty,
                    result: AssertionResult::Unprocessable(
                        UnprocessableReason::SerializationFailure(err.to_string()),
                    ),
                }
            }
        };
        self.is_eq(&expected)
    }

    fn is_ne(&self, expected: &String) -> Self::Assertion {
        let expected: Value = match serde_json::from_str(expected) {
            Ok(value) => value,
            Err(err) => {
                return Assertion {
                    predicate: Predicate::IsNot,
                    part: Part::JsonPath,
                    left: Hand::Empty,
                    right: Hand::Empty,
                    result: AssertionResult::Unprocessable(
                        UnprocessableReason::SerializationFailure(err.to_string()),
                    ),
                }
            }
        };
        self.is_ne(&expected)
    }
}

impl Equality<str> for JsonPathResult<'_, Value> {
    type Assertion = Assertion<Value>;

    fn is_eq(&self, expected: &str) -> Self::Assertion {
        let expected: Value = match serde_json::from_str(expected) {
            Ok(value) => value,
            Err(err) => {
                return Assertion {
                    predicate: Predicate::Is,
                    part: Part::JsonPath,
                    left: Hand::Empty,
                    right: Hand::Empty,
                    result: AssertionResult::Unprocessable(
                        UnprocessableReason::SerializationFailure(err.to_string()),
                    ),
                }
            }
        };
        self.is_eq(&expected)
    }

    fn is_ne(&self, expected: &str) -> Self::Assertion {
        let expected: Value = match serde_json::from_str(expected) {
            Ok(value) => value,
            Err(err) => {
                return Assertion {
                    predicate: Predicate::IsNot,
                    part: Part::JsonPath,
                    left: Hand::Empty,
                    right: Hand::Empty,
                    result: AssertionResult::Unprocessable(
                        UnprocessableReason::SerializationFailure(err.to_string()),
                    ),
                }
            }
        };
        self.is_ne(&expected)
    }
}

impl Equality<PathBuf> for JsonPathResult<'_, Value> {
    type Assertion = Assertion<Value>;

    fn is_eq(&self, json_file: &PathBuf) -> Self::Assertion {
        let json_file = match fs::read_to_string(json_file) {
            Ok(content) => content,
            Err(_) => {
                return Assertion {
                    predicate: Predicate::Is,
                    part: Part::JsonPath,
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
                    part: Part::JsonPath,
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
                    part: Part::JsonPath,
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
                    part: Part::JsonPath,
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

impl JsonSchema<Value> for JsonPathResult<'_, Value> {
    type Assertion = Assertion<Value>;

    fn matches_schema(&self, schema: &Value) -> Self::Assertion {
        let schema = match JSONSchema::compile(schema) {
            Ok(schema) => schema,
            Err(err) => {
                return Assertion {
                    predicate: Predicate::Schema,
                    part: Part::JsonPath,
                    left: Hand::Compound(Value::String(self.path.to_string()), self.value.clone()),
                    right: Hand::Empty,
                    result: AssertionResult::Unprocessable(UnprocessableReason::InvalidJsonSchema(
                        err.schema_path,
                        err.instance_path,
                    )),
                }
            }
        };

        // Get the boolean result of the validation
        let result = schema.is_valid(&self.value);

        // Generate a json output of the json schema result
        let output: BasicOutput<'_> = schema.apply(&self.value).basic();
        let output = match serde_json::to_value(output) {
            Ok(json_output) => json_output,
            Err(_) => {
                return Assertion {
                    predicate: Predicate::Schema,
                    part: Part::JsonPath,
                    left: Hand::Compound(Value::String(self.path.to_string()), self.value.clone()),
                    right: Hand::Empty,
                    result: AssertionResult::Unprocessable(UnprocessableReason::Other(
                        "Failed to serialize json schema error".to_string(),
                    )),
                }
            }
        };

        Assertion {
            predicate: Predicate::Schema,
            part: Part::JsonPath,
            left: Hand::Compound(Value::String(self.path.to_string()), self.value.clone()),
            right: Hand::Right(output),
            result: result.into(),
        }
    }
}

impl JsonSchema<str> for JsonPathResult<'_, Value> {
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

impl JsonSchema<String> for JsonPathResult<'_, Value> {
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

impl JsonSchema<PathBuf> for JsonPathResult<'_, Value> {
    type Assertion = Assertion<Value>;

    fn matches_schema(&self, schema_file: &PathBuf) -> Self::Assertion {
        let schema_file_content = match fs::read_to_string(schema_file) {
            Ok(content) => content,
            Err(_) => {
                return Assertion {
                    predicate: Predicate::Schema,
                    part: Part::JsonPath,
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
                    part: Part::JsonPath,
                    left: Hand::Empty,
                    right: Hand::Empty,
                    result: AssertionResult::Unprocessable(
                        UnprocessableReason::SerializationFailure(
                            "Failed to serialize json schema".to_string(),
                        ),
                    ),
                }
            }
        };

        self.matches_schema(&schema)
    }
}

/// Make sure the given `Value` will be a `Value::Array` variant
/// to compare with a `JsonPathResult`; a wrapper around the result
/// returned by the jsonpath library.
///
/// The value returned for a given path is always an array, even if there
/// is only one element. This is because the jsonpath library doesn't know in
/// advance how many items will be returned.
fn to_value_array(value: &Value) -> Value {
    Value::Array(vec![value.clone()])
}

#[cfg(test)]
mod tests {
    use crate::dsl::json_path::JsonPathResult;
    use serde_json::{json, Value};

    fn json_stub() -> Value {
        json!({
          "shop": {
            "orders": [
              {
                "id": 1,
                "active": true
              },
              {
                "id": 2
              },
              {
                "id": 3
              },
              {
                "id": 4,
                "active": true
              }
            ],
            "total": 4
          }
        })
    }

    // This module test equality through different data structures to also
    // cover the `Value::Array` wrapper.
    mod is_eq {
        use super::{json_stub, JsonPathResult};
        use crate::assertion::traits::Equality;
        use jsonpath_rust::JsonPathQuery;
        use serde_json::json;
        use std::path::PathBuf;

        #[test]
        fn impl_is_eq_should_fail_with_inexistant_data() {
            let path = "$.unknown";
            let value = json_stub().path(path).unwrap();
            let jsonpath_result = JsonPathResult { path, value };

            let assertion = jsonpath_result.is_eq(&json_stub());
            assert!(assertion.failed(), "{}", assertion.log());
        }

        #[test]
        fn impl_is_eq_root() {
            let path = "$";
            let value = json_stub().path(path).unwrap();
            let jsonpath_result = JsonPathResult { path, value };

            let assertion = jsonpath_result.is_eq(&json_stub());
            assert!(assertion.passed(), "{}", assertion.log());
        }

        #[test]
        fn impl_is_eq_one_array() {
            let path = "$.shop.orders";
            let value = json_stub().path(path).unwrap();
            let jsonpath_result = JsonPathResult { path, value };
            let expected_json = json!([
              {
                "id": 1,
                "active": true
              },
              {
                "id": 2
              },
              {
                "id": 3
              },
              {
                "id": 4,
                "active": true
              }
            ]);

            let assertion = jsonpath_result.is_eq(&expected_json);
            assert!(assertion.passed(), "{}", assertion.log());
        }

        #[test]
        fn impl_is_eq_one_object() {
            let path = "$.shop.orders[0]";
            let value = json_stub().path(path).unwrap();
            let jsonpath_result = JsonPathResult { path, value };
            let expected_json = json!({
              "id": 1,
              "active": true
            });

            let assertion = jsonpath_result.is_eq(&expected_json);
            assert!(assertion.passed(), "{}", assertion.log());
        }

        #[test]
        fn impl_is_eq_object_with_array_and_object() {
            let path = "$.shop";
            let value = json_stub().path(path).unwrap();
            let jsonpath_result = JsonPathResult { path, value };
            let expected_json = json!({
                "orders": [
                    {
                        "id": 1,
                        "active": true
                    },
                    {
                        "id": 2
                    },
                    {
                        "id": 3
                    },
                    {
                        "id": 4,
                        "active": true
                    }
                ],
                "total": 4
            });

            let assertion = jsonpath_result.is_eq(&expected_json);
            assert!(assertion.passed(), "{}", assertion.log());
        }

        #[test]
        fn impl_is_eq_object_with_number() {
            let path = "$.shop.total";
            let value = json_stub().path(path).unwrap();
            let jsonpath_result = JsonPathResult { path, value };
            let expected_json = json!(4);

            let assertion = jsonpath_result.is_eq(&expected_json);
            assert!(assertion.passed(), "{}", assertion.log());
        }

        #[test]
        fn impl_is_eq_object_with_number_type_different_fails() {
            let path = "$.shop.total";
            let value = json_stub().path(path).unwrap();
            let jsonpath_result = JsonPathResult { path, value };

            // float
            let assertion = jsonpath_result.is_eq(&json!(4.0));
            assert!(assertion.failed(), "{}", assertion.log());

            // string
            let assertion = jsonpath_result.is_eq(&json!("4"));
            assert!(assertion.failed(), "{}", assertion.log());
        }

        #[test]
        fn impl_is_eq_json_file() {
            let path = "$.shop";
            let value = json_stub().path(path).unwrap();
            let jsonpath_result = JsonPathResult { path, value };
            let json_file =
                PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("tests/fixtures/orders.json");

            let assertion = jsonpath_result.is_eq(&json_file);
            assert!(assertion.passed(), "{}", assertion.log());
        }

        #[test]
        fn impl_is_eq_inexistant_json_file() {
            let path = "$.shop.orders[0]";
            let value = json_stub().path(path).unwrap();
            let jsonpath_result = JsonPathResult { path, value };
            let schema_file_path =
                PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("tests/fixtures/inexistant.json");

            let assertion = jsonpath_result.is_eq(&schema_file_path);
            assert!(assertion.failed(), "{}", assertion.log());
            let expected_error_msg_part = "Failed to read json file located at";

            assert!(
                assertion.log().contains(expected_error_msg_part),
                "The error message doesn't contain this: {expected_error_msg_part}"
            );
        }
    }

    mod is_ne {
        use super::{json_stub, JsonPathResult};
        use crate::assertion::traits::Equality;
        use jsonpath_rust::JsonPathQuery;
        use serde_json::json;
        use std::path::PathBuf;

        #[test]
        fn impl_is_ne_object_with_array_and_object() {
            let path = "$.shop";
            let value = json_stub().path(path).unwrap();
            let jsonpath_result = JsonPathResult { path, value };
            // commented element highlights what was removed from initial
            // json data.
            let expected_json = json!({
                "orders": [
                    {
                        "id": 1,
                        // "active": true
                    },
                    {
                        "id": 2
                    },
                    {
                        "id": 3
                    },
                    {
                        "id": 4,
                        "active": true
                    }
                ],
                "total": 4
            });

            let assertion = jsonpath_result.is_ne(&expected_json);
            assert!(assertion.passed(), "{}", assertion.log());
        }

        #[test]
        fn impl_is_ne_fails() {
            let path = "$.shop";
            let value = json_stub().path(path).unwrap();
            let jsonpath_result = JsonPathResult { path, value };
            let expected_json = json!({
                "orders": [
                    {
                        "id": 1,
                        "active": true
                    },
                    {
                        "id": 2
                    },
                    {
                        "id": 3
                    },
                    {
                        "id": 4,
                        "active": true
                    }
                ],
                "total": 4
            });

            let assertion = jsonpath_result.is_ne(&expected_json);
            // Fails because asserted value is equals to the expected value.
            assert!(assertion.failed(), "{}", assertion.log());
        }

        #[test]
        fn impl_is_ne_json_file() {
            let path = "$";
            let value = json_stub().path(path).unwrap();
            let jsonpath_result = JsonPathResult { path, value };
            let json_file =
                PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("tests/fixtures/orders.json");

            let assertion = jsonpath_result.is_ne(&json_file);
            assert!(assertion.passed(), "{}", assertion.log());
        }
    }

    mod json_schema {
        use std::path::PathBuf;

        use super::{json, json_stub, JsonPathResult};
        use crate::assertion::traits::JsonSchema;
        use jsonpath_rust::JsonPathQuery;

        #[test]
        fn impl_json_schema_is_valid() {
            let path = "$.shop.orders[0]";
            let value = json_stub().path(path).unwrap();
            let jsonpath_result = JsonPathResult { path, value };
            let schema = json!({
              "$schema": "http://json-schema.org/draft-04/schema#",
              "title": "Order validation schema",
              "type": "array",
              "items": {
                "type": "object",
                "required": ["id", "active"],
                "properties": {
                    "id": {
                      "type": "number"
                    },
                    "active": {
                        "type": "boolean"
                    }
                  },
              }
            });

            let assertion = jsonpath_result.matches_schema(&schema);
            assert!(assertion.passed(), "{}", assertion.log());
        }

        #[test]
        fn impl_json_schema_is_valid_str_or_string() {
            let path = "$.shop.orders[0]";
            let value = json_stub().path(path).unwrap();
            let jsonpath_result = JsonPathResult { path, value };
            let schema = r#"{
              "$schema": "http://json-schema.org/draft-04/schema#",
              "title": "Order validation schema",
              "type": "array",
              "items": {
                "type": "object",
                "required": ["id", "active"],
                "properties": {
                    "id": {
                      "type": "number"
                    },
                    "active": {
                        "type": "boolean"
                    }
                  }
              }
            }"#;

            let assertion = jsonpath_result.matches_schema(schema);
            assert!(assertion.passed(), "{}", assertion.log());

            let assertion = jsonpath_result.matches_schema(&schema.to_string());
            assert!(assertion.passed(), "{}", assertion.log());
        }

        #[test]
        fn impl_json_schema_is_invalid() {
            let path = "$.shop.orders[3]";
            let value = json_stub().path(path).unwrap();
            let jsonpath_result = JsonPathResult { path, value };
            let schema = json!({
              "$schema": "http://json-schema.org/draft-04/schema#",
              "title": "Order validation schema",
              "type": "array",
              "items": {
                "type": "object",
                "required": ["id", "active"],
                "properties": {
                    "id": {
                      "type": "integer"
                    },
                    "active": {
                        "type": "string" // set the wrong type on purpose, we receive a boolean
                    }
                  },
              }
            });

            let assertion = jsonpath_result.matches_schema(&schema);
            assert!(assertion.failed(), "{}", assertion.log());
        }

        #[test]
        fn impl_json_schema_file() {
            let path = "$.shop.orders[0]";
            let value = json_stub().path(path).unwrap();
            let jsonpath_result = JsonPathResult { path, value };
            let schema_file_path =
                PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("tests/fixtures/orders_schema.json");

            let assertion = jsonpath_result.matches_schema(&schema_file_path);
            assert!(assertion.passed(), "{}", assertion.log());
        }

        #[test]
        fn impl_json_schema_inexistant_file() {
            let path = "$.shop.orders[0]";
            let value = json_stub().path(path).unwrap();
            let jsonpath_result = JsonPathResult { path, value };
            let schema_file_path =
                PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("tests/fixtures/inexistant.json");

            let assertion = jsonpath_result.matches_schema(&schema_file_path);
            assert!(assertion.failed(), "{}", assertion.log());
            let expected_error_msg_part = "Failed to read json schema file located at";

            assert!(
                assertion.log().contains(expected_error_msg_part),
                "The error message doesn't contain this: {expected_error_msg_part}"
            );
        }
    }
    mod serialization {
        use super::{json_stub, JsonPathResult};
        use crate::assertion::traits::Equality;
        use jsonpath_rust::JsonPathQuery;
        use serde_json::json;

        #[test]
        fn it_serializes_jsonpath_should_be() {
            let path = "$.shop.orders[0]";
            let value = json_stub().path(path).unwrap();
            let jsonpath_result = JsonPathResult { path, value };
            let json_path_value = json!({
              "id": 1,
              "active": true
            });

            let expected_serialization = json!({
                "part": "json path",
                "predicate": "should be",
                "left":  json!([
                    path,
                    [json_path_value]
                    ]),
                "right": [json_path_value],
                "result": "passed"
            });

            let assertion = jsonpath_result.is_eq(&json_path_value);

            assert_eq!(
                json!(assertion),
                expected_serialization,
                "Serialized assertion is not equals to the expected json {:#?}",
                assertion
            );
        }

        #[test]
        fn it_serializes_jsonpath_should_not_be() {
            let path = "$.shop.orders[0]";
            let value_array = json_stub().path(path).unwrap();
            let jsonpath_result = JsonPathResult {
                path,
                value: value_array.clone(),
            };
            let expected_not_json_path_value = json!({
                "id": 1,
                "active": false
            });

            let expected_serialization = json!({
                "part": "json path",
                "predicate": "should not be",
                "left":  json!([
                    path,
                    value_array
                    ]),
                "right": [expected_not_json_path_value],
                "result": "passed"
            });

            let assertion = jsonpath_result.is_ne(&expected_not_json_path_value);

            assert_eq!(
                json!(assertion),
                expected_serialization,
                "Serialized assertion is not equals to the expected json {:#?}",
                assertion
            );
        }
    }
}
