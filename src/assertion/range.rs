use http::StatusCode;

use crate::assertion::{Assertion, Hand};
use crate::dsl::{Part, Predicate};

trait IsBetween<T> {
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
    fn is_between(self, min: T, max: T) -> Self::Assertion;
}

macro_rules! is_between_impl {
    ($($t:ty)*) => ($(
        impl IsBetween<$t> for $t {
            type Assertion = Assertion<$t>;

            fn is_between(self, min: $t, max: $t) -> Assertion<$t> {
                let result = self > min && self < max;
                Assertion {
                    predicate: Predicate::Between,
                    part: Part::Empty,
                    left: Hand::Left(self),
                    right: Hand::Range(min, max),
                    result: result.into(),
                }
            }
        }
    )*)
}

is_between_impl!(u128 StatusCode);

#[cfg(test)]
mod tests {
    use crate::{assertion::range::IsBetween, StatusCode};

    #[test]
    fn u128_blanket_impls() {
        assert!(15_000_u128.is_between(u128::MIN, u128::MAX).passed());
    }
}
