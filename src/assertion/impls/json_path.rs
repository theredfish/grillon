use crate::{
    assertion::{traits::Equality, Assertion, Hand},
    dsl::{json_path::JsonPathResult, Part, Predicate},
};
use serde_json::Value;

// Note that lhs and rhs are inversed because of the JsonPathResult.
impl Equality<Value> for JsonPathResult<'_, Value> {
    type Assertion = Assertion<Value>;

    fn is_eq(&self, actual: &Value) -> Self::Assertion {
        let result = &self.value == actual;

        Assertion {
            predicate: Predicate::Is,
            part: Part::JsonPath,
            left: Hand::Left(actual.clone()),
            right: Hand::Compound(Value::String(self.path.to_string()), self.value.clone()),
            result: result.into(),
        }
    }

    fn is_ne(&self, actual: &Value) -> Self::Assertion {
        let result = &self.value != actual;

        Assertion {
            predicate: Predicate::IsNot,
            part: Part::JsonPath,
            left: Hand::Left(actual.clone()),
            right: Hand::Compound(Value::String(self.path.to_string()), self.value.clone()),
            result: result.into(),
        }
    }
}
