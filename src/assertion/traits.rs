/// Trait to test the equality between two values.
pub trait IsEq<Rhs = Self> {
    /// The resulting assertion after applying the equality test.
    type Assertion;

    /// Asserts the equality.
    fn is_eq(self, rhs: Rhs) -> Self::Assertion;
}

/// Trait to test the non equality between two values.
pub trait IsNe<Rhs = Self>: PartialEq {
    /// The resulting assertion after applying the non equality test.
    type Assertion;

    /// Asserts the non equality.
    fn is_ne(self, rhs: Rhs) -> Self::Assertion;
}

/// Trait to test if a value is withing an inclusive range.
pub trait RangeInclusive<T> {
    /// The resulting assertion after applying the inclusive range test.
    type Assertion;

    /// Asserts the value is in open range.
    fn in_range(self, min: T, max: T) -> Self::Assertion;
}
