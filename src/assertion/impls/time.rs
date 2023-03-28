use crate::{
    assertion::{traits::LessThan, Assertion, Hand},
    dsl::{Part, Predicate},
};

impl LessThan<u64> for u64 {
    type Assertion = Assertion<u64>;

    fn less_than(&self, other: &u64) -> Self::Assertion {
        let result = self < other;

        Assertion {
            part: Part::ResponseTime,
            predicate: Predicate::LessThan,
            left: Hand::Left(*self),
            right: Hand::Right(*other),
            result: result.into(),
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::assertion::traits::LessThan;

    #[test]
    fn it_should_be_less_than() {
        let assertion = 20_u64.less_than(&30);
        assert!(assertion.passed(), "{}", assertion.log());
    }

    #[test]
    fn it_should_not_be_less_than() {
        let assertion = u64::MAX.less_than(&30);
        assert!(assertion.failed(), "{}", assertion.log());
    }

    mod serialization {
        use super::*;
        use serde_json::json;

        #[test]
        fn it_serializes_time_less_than() {
            let expected_json = json!({
                "part": "response time",
                "predicate": "should be less than",
                "left": 200_u64,
                "right": 300_u64,
                "result": "passed"
            });

            let assertion = 200.less_than(&300);

            assert_eq!(
                json!(assertion),
                expected_json,
                "Serialized assertion is not equals to the expected json",
            );
        }
    }
}
