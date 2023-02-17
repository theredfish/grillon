//! This module contains various traits for comparisons, range checks and inner
//! checks.

use crate::dsl::Expression;

/// Trait to test the equality between two values.
pub trait Equality<Rhs: ?Sized = Self> {
    /// The resulting assertion after applying the equality test.
    type Assertion;

    /// Asserts the equality.
    fn is_eq(&self, rhs: &Rhs) -> Self::Assertion;

    /// Asserts the non equality.
    fn is_ne(&self, rhs: &Rhs) -> Self::Assertion;
}

/// Trait to test if a value is withing an inclusive range.
pub trait RangeInclusive<T: ?Sized> {
    /// The resulting assertion after applying the inclusive range test.
    type Assertion;

    /// Asserts the value is in open range.
    fn in_range(&self, min: &T, max: &T) -> Self::Assertion;
}

/// Trait to test if a value is less than the other.
pub trait LessThan<T: ?Sized> {
    /// The resulting assertion after applying the less than test.
    type Assertion;

    /// Asserts the value is in open range.
    fn less_than(&self, other: &T) -> Self::Assertion;
}

/// A representation of a container of items where we can perform inner checks
/// with `has` and `has_not` functions.
pub trait Container<T: ?Sized> {
    /// The resulting assertion after applying the contains test.
    type Assertion;

    /// Asserts that the container contains other.
    fn has(&self, other: &T) -> Self::Assertion;

    /// Asserts that the container does not contain other.
    fn has_not(&self, other: &T) -> Self::Assertion;
}

/// Trait to test a value for a given json path.
pub trait JsonPath<T: ?Sized> {
    /// The resulting assertion after applying the json path test.
    type Assertion;

    /// Asserts the value found, at the given json path, matches the provided assertion expression.
    fn json_path(&self, selector: String, expression: Expression<&T>) -> Self::Assertion;
}
