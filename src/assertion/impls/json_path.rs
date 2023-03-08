use crate::{
    assertion::{traits::Equality, Assertion, Hand},
    dsl::{json_path::JsonPathResult, Part, Predicate},
};
use serde_json::Value;

// Note that lhs and rhs are inversed here because of the JsonPathResult wrapper.
// We can't test equality in the other way since the trait Equality is
// already implemented for json (which is a Value). Might change later.
impl Equality<Value> for JsonPathResult<'_, Value> {
    type Assertion = Assertion<Value>;

    fn is_eq(&self, actual: &Value) -> Self::Assertion {
        let result = self.value == to_value_array(actual);

        Assertion {
            predicate: Predicate::Is,
            part: Part::JsonPath,
            left: Hand::Left(actual.clone()),
            right: Hand::Compound(Value::String(self.path.to_string()), self.value.clone()),
            result: result.into(),
        }
    }

    fn is_ne(&self, actual: &Value) -> Self::Assertion {
        let result = self.value != to_value_array(actual);

        Assertion {
            predicate: Predicate::IsNot,
            part: Part::JsonPath,
            left: Hand::Left(actual.clone()),
            right: Hand::Compound(Value::String(self.path.to_string()), self.value.clone()),
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

    mod is_eq {
        use super::json_stub;
        use crate::{assertion::traits::Equality, dsl::json_path::JsonPathResult};
        use jsonpath_rust::JsonPathQuery;
        use serde_json::json;

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

    // TODO: test is_ne, de/ser, and assertion failures with invalid path
}
