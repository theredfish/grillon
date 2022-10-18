use crate::assertion::traits::{Equality, RangeInclusive};
use crate::assertion::{Assertion, Hand};
use crate::dsl::{Part, Predicate};
use crate::StatusCode;

impl Equality<u16> for StatusCode {
    type Assertion = Assertion<u16>;

    fn is_eq(&self, rhs: &u16) -> Self::Assertion {
        let lhs = self.as_u16();

        Assertion {
            predicate: Predicate::Is,
            part: Part::StatusCode,
            left: Hand::Left(lhs),
            right: Hand::Right(*rhs),
            result: (self == rhs).into(),
        }
    }

    fn is_ne(&self, rhs: &u16) -> Self::Assertion {
        let lhs = self.as_u16();

        Assertion {
            predicate: Predicate::IsNot,
            part: Part::StatusCode,
            left: Hand::Left(lhs),
            right: Hand::Right(*rhs),
            result: (self != rhs).into(),
        }
    }
}

impl Equality<StatusCode> for StatusCode {
    type Assertion = Assertion<u16>;

    fn is_eq(&self, rhs: &StatusCode) -> Self::Assertion {
        Assertion {
            predicate: Predicate::Is,
            part: Part::StatusCode,
            left: Hand::Left(self.as_u16()),
            right: Hand::Right(rhs.as_u16()),
            result: (self == rhs).into(),
        }
    }

    fn is_ne(&self, rhs: &StatusCode) -> Self::Assertion {
        Assertion {
            predicate: Predicate::IsNot,
            part: Part::StatusCode,
            left: Hand::Left(self.as_u16()),
            right: Hand::Right(rhs.as_u16()),
            result: (self != rhs).into(),
        }
    }
}

impl RangeInclusive<StatusCode> for StatusCode {
    type Assertion = Assertion<u16>;

    fn in_range(&self, min: &StatusCode, max: &StatusCode) -> Self::Assertion {
        let lhs = self.as_u16();
        let (min, max) = (min.as_u16(), max.as_u16());
        let result = lhs >= min && lhs <= max;

        Assertion {
            predicate: Predicate::Between,
            part: Part::StatusCode,
            left: Hand::Left(lhs),
            right: Hand::Range(min, max),
            result: result.into(),
        }
    }
}

impl RangeInclusive<u16> for StatusCode {
    type Assertion = Assertion<u16>;

    fn in_range(&self, min: &u16, max: &u16) -> Self::Assertion {
        let lhs = self.as_u16();
        let result = &lhs >= min && &lhs <= max;

        Assertion {
            predicate: Predicate::Between,
            part: Part::StatusCode,
            left: Hand::Left(lhs),
            right: Hand::Range(*min, *max),
            result: result.into(),
        }
    }
}

#[cfg(test)]
pub mod tests {
    use http::StatusCode;

    use crate::assertion::traits::{Equality, RangeInclusive};

    #[test]
    fn impl_is_eq_status_code() {
        let assertion = StatusCode::FORBIDDEN.is_eq(&StatusCode::FORBIDDEN);
        assert!(assertion.passed(), "{}", assertion.message())
    }

    #[test]
    fn impl_is_eq_u16() {
        let assertion = StatusCode::FORBIDDEN.is_eq(&403);
        assert!(assertion.passed(), "{}", assertion.message())
    }

    #[test]
    fn impl_is_not_status_code() {
        let assertion = StatusCode::FORBIDDEN.is_ne(&StatusCode::OK);
        assert!(assertion.passed(), "{}", assertion.message())
    }

    #[test]
    fn impl_is_not_u16() {
        let assertion = StatusCode::FORBIDDEN.is_ne(&200);
        assert!(assertion.passed(), "{}", assertion.message())
    }

    #[test]
    fn impl_is_between_status_code() {
        let assertion =
            StatusCode::FORBIDDEN.in_range(&StatusCode::BAD_REQUEST, &StatusCode::NOT_FOUND);

        assert!(assertion.passed(), "{}", assertion.message())
    }

    #[test]
    fn impl_is_between_u16() {
        assert!(StatusCode::FORBIDDEN.in_range(&400, &404).passed())
    }

    mod serialization {
        use crate::assertion::Hand;

        use super::*;
        use serde_json::json;

        #[test]
        fn it_serializes_status_should_be() {
            let status = StatusCode::UNAUTHORIZED;

            let expected_json = json!({
                "part": "status code",
                "predicate": "should be",
                "left": status.as_u16(),
                "right": 401,
                "result": "passed"
            });

            let assertion = status.is_eq(&401);

            assert_eq!(
                json!(assertion),
                expected_json,
                "Serialized assertion is not equals to the expected json",
            );
        }

        #[test]
        fn it_serializes_status_should_not_be() {
            let status = StatusCode::UNAUTHORIZED;

            let expected_json = json!({
                "part": "status code",
                "predicate": "should not be",
                "left": status.as_u16(),
                "right": 404,
                "result": "passed"
            });

            let assertion = status.is_ne(&404);

            assert_eq!(
                json!(assertion),
                expected_json,
                "Serialized assertion is not equals to the expected json",
            );
        }

        #[test]
        fn it_serializes_status_is_between() {
            let status = StatusCode::UNAUTHORIZED;

            let expected_json = json!({
                "part": "status code",
                "predicate": "should be between",
                "left": status.as_u16(),
                "right": Hand::Range(400, 404),
                "result": "passed"
            });

            let assertion = status.in_range(&400, &404);

            assert_eq!(
                json!(assertion),
                expected_json,
                "Serialized assertion is not equals to the expected json",
            );
        }
    }
}
