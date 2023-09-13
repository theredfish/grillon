use crate::{
    assertion::{traits::Equality, Assertion, Hand},
    dsl::{json_path::JsonPathResult, Part, Predicate},
};
use serde_json::Value;

// Hand compound here is used to store the json path result made
// of a path and of the value found at this given path.
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
    }

    mod is_ne {
        use super::{json_stub, JsonPathResult};
        use crate::assertion::traits::Equality;
        use jsonpath_rust::JsonPathQuery;
        use serde_json::json;

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
