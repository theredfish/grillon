use crate::assertion::traits::{IsEq, IsNe, RangeInclusive};
use crate::assertion::{Assertion, Hand};
use crate::dsl::{Part, Predicate};
use crate::StatusCode;

impl IsEq<u16> for StatusCode {
    type Assertion = Assertion<u16>;

    fn is_eq(self, rhs: u16) -> Self::Assertion {
        let lhs = self.as_u16();

        Assertion {
            predicate: Predicate::Is,
            part: Part::StatusCode,
            left: Hand::Left(lhs),
            right: Hand::Right(rhs),
            result: (self == rhs).into(),
        }
    }
}

impl IsEq<StatusCode> for StatusCode {
    type Assertion = Assertion<u16>;

    fn is_eq(self, rhs: StatusCode) -> Self::Assertion {
        Assertion {
            predicate: Predicate::Is,
            part: Part::StatusCode,
            left: Hand::Left(self.as_u16()),
            right: Hand::Right(rhs.as_u16()),
            result: (self == rhs).into(),
        }
    }
}

impl IsNe<u16> for StatusCode {
    type Assertion = Assertion<u16>;

    fn is_ne(self, rhs: u16) -> Self::Assertion {
        let lhs = self.as_u16();

        Assertion {
            predicate: Predicate::IsNot,
            part: Part::StatusCode,
            left: Hand::Left(lhs),
            right: Hand::Right(rhs),
            result: (self != rhs).into(),
        }
    }
}

impl IsNe<StatusCode> for StatusCode {
    type Assertion = Assertion<u16>;

    fn is_ne(self, rhs: StatusCode) -> Self::Assertion {
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

    fn in_range(self, min: StatusCode, max: StatusCode) -> Self::Assertion {
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

    fn in_range(self, min: u16, max: u16) -> Self::Assertion {
        let lhs = self.as_u16();
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

#[cfg(test)]
pub mod tests {
    use http::StatusCode;

    use crate::assertion::traits::{IsEq, IsNe, RangeInclusive};

    #[test]
    fn impl_is_status_code() {
        assert!(StatusCode::FORBIDDEN.is_eq(StatusCode::FORBIDDEN).passed())
    }

    #[test]
    fn impl_is_u16() {
        assert!(StatusCode::FORBIDDEN.is_eq(403).passed())
    }

    #[test]
    fn impl_is_not_status_code() {
        assert!(StatusCode::FORBIDDEN.is_ne(StatusCode::OK).passed())
    }

    #[test]
    fn impl_is_not_u16() {
        assert!(StatusCode::FORBIDDEN.is_ne(200).passed())
    }

    #[test]
    fn impl_is_between_status_code() {
        assert!(StatusCode::FORBIDDEN
            .in_range(StatusCode::BAD_REQUEST, StatusCode::NOT_FOUND)
            .passed())
    }

    #[test]
    fn impl_is_between_u16() {
        assert!(StatusCode::FORBIDDEN.in_range(400, 404).passed())
    }
}
