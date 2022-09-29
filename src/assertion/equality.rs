use crate::assertion::{Assertion, Hand};
use crate::dsl::{Part, Predicate};
use crate::StatusCode;

trait Is<Rhs = Self> {
    /// The resulting assertion after applying the equality test
    type Assertion;

    /// Asserts the equality.
    ///
    /// # Example
    ///
    /// ```
    /// let x = 10_u16;
    /// assert!(10.is(x));
    /// ```
    fn is(self, rhs: Rhs) -> Self::Assertion;
}

macro_rules! is_impl {
    ($($t:ty)*) => ($(
        impl Is for $t {
            type Assertion = Assertion<$t>;

            fn is(self, other: $t) -> Assertion<$t> {
                Assertion {
                    predicate: Predicate::Is,
                    part: Part::Empty,
                    left: Hand::Left(self),
                    right: Hand::Right(other),
                    result: (self == other).into(),
                }
            }
        }
    )*)
}

is_impl! { StatusCode u16 u128 }

trait IsNot<Rhs = Self>: PartialEq {
    /// The resulting assertion after applying the equality test
    type Assertion;

    /// Asserts the non equality.
    ///
    /// # Example
    ///
    /// ```
    /// let x = 20_u16;
    /// assert!(10.is_not(x));
    /// ```
    fn is_not(self, rhs: Rhs) -> Self::Assertion;
}

macro_rules! is_not_impl {
    ($($t:ty)*) => ($(
        impl IsNot for $t {
            type Assertion = Assertion<$t>;

            fn is_not(self, other: $t) -> Assertion<$t> {
                Assertion {
                    predicate: Predicate::IsNot,
                    part: Part::Empty,
                    left: Hand::Left(self),
                    right: Hand::Right(other),
                    result: (self != other).into(),
                }
            }
        }
    )*)
}

is_not_impl! { StatusCode u16 u128 }

#[cfg(test)]
mod tests {
    use super::{Assertion, Hand, Is, IsNot};
    use crate::dsl::{Part, Predicate};
    use crate::StatusCode;

    #[test]
    fn u8_blanket_impls() {
        assert!(u16::MAX.is(u16::MAX).passed());
        assert!(u16::MAX.is(u16::MIN).failed());

        assert!(u16::MAX.is_not(u16::MAX).failed());
        assert!(u16::MAX.is_not(u16::MIN).passed());
    }

    #[test]
    fn u128_blanket_impls() {
        assert!(u128::MAX.is(u128::MAX).passed());
        assert!(u128::MAX.is(u128::MIN).failed());

        assert!(u128::MAX.is_not(u128::MAX).failed());
        assert!(u128::MAX.is_not(u128::MIN).passed());
    }

    #[test]
    fn status_code_blanket_impls() {
        assert!(StatusCode::OK.is(StatusCode::OK).passed());
        assert!(StatusCode::OK
            .is(StatusCode::INTERNAL_SERVER_ERROR)
            .failed());

        assert!(StatusCode::OK.is_not(StatusCode::OK).failed());
        assert!(StatusCode::OK
            .is_not(StatusCode::INTERNAL_SERVER_ERROR)
            .passed());
    }

    #[test]
    fn u16_is_status_code_impl() {
        impl Is<StatusCode> for u16 {
            type Assertion = Assertion<u16>;

            fn is(self, rhs: StatusCode) -> Self::Assertion {
                let rhs = rhs.as_u16();

                Assertion {
                    predicate: Predicate::Is,
                    part: Part::StatusCode,
                    left: Hand::Left(self),
                    right: Hand::Right(rhs),
                    result: (self == rhs).into(),
                }
            }
        }

        assert!(
            200.is(StatusCode::OK).passed(),
            "Failed to assert that {} is equals to 200",
            StatusCode::OK
        );
    }

    #[test]
    fn u16_is_not_status_code_impl() {
        impl IsNot<StatusCode> for u16 {
            type Assertion = Assertion<u16>;

            fn is_not(self, rhs: StatusCode) -> Self::Assertion {
                let rhs = rhs.as_u16();

                Assertion {
                    predicate: Predicate::IsNot,
                    part: Part::StatusCode,
                    left: Hand::Left(self),
                    right: Hand::Right(rhs),
                    result: (self != rhs).into(),
                }
            }
        }

        assert!(
            200.is_not(StatusCode::INTERNAL_SERVER_ERROR).passed(),
            "Failed to assert that {} is not equals to 200",
            StatusCode::OK
        );
    }
}
