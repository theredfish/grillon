use crate::dsl::{Part, Predicate};
#[cfg(feature = "diff")]
use pretty_assertions::assert_eq;
use std::fmt::Debug;

/// Defines assertion behaviors to implement for a concrete type of assertion.
pub trait AssertKind {
    /// Runs the assertion with a [`Predicate`] and a [`Part`].
    fn assert(&self, predicate: &Predicate, part: &Part);
    /// Builds the assertion message with a [`Predicate`] and a [`Part`].
    fn message(&self, predicate: &Predicate, part: &Part) -> String;
}

/// Represents an assertion to test the equality between `left` and `right`. If
/// the `diff` feature is enabled, the underlying macro `assert_eq!` will
/// generate a diff for assertion failures.
pub struct AssertEq<T> {
    /// The left-hand operand under strict equality test.
    pub left: T,
    /// The right-hand operand under strict equality test.
    pub right: T,
}

impl<T> AssertKind for AssertEq<T>
where
    T: PartialEq + Debug,
{
    fn assert(&self, predicate: &Predicate, part: &Part) {
        assert_eq!(self.left, self.right, "{}", self.message(predicate, part))
    }

    fn message(&self, predicate: &Predicate, part: &Part) -> String {
        format!(
            "{} {} {:#?}. Found {:#?}.",
            part, predicate, self.right, self.left
        )
    }
}

/// Represents an assertion to test the non-equality between `left` and `right`.
pub struct AssertNe<T> {
    /// The left-hand operand under strict non-equality test.
    pub left: T,
    /// The right-hand operand under strict non-equality test.
    pub right: T,
}

impl<T> AssertKind for AssertNe<T>
where
    T: PartialEq + Debug,
{
    fn assert(&self, predicate: &Predicate, part: &Part) {
        assert_ne!(self.left, self.right, "{}", self.message(predicate, part))
    }

    fn message(&self, predicate: &Predicate, part: &Part) -> String {
        format!(
            "{} {} {:#?}. Found {:#?}.",
            part, predicate, self.right, self.left
        )
    }
}

/// Represents an assertion between `left` and `right` with a test expression.
pub struct AssertBool<T, U> {
    /// The left-hand operand under test.
    pub left: T,
    /// The right-hand operand under test.
    pub right: U,
    /// The test expression result.
    pub result: bool,
}

impl<T, U> AssertKind for AssertBool<T, U>
where
    T: PartialEq + Debug,
    U: PartialEq + Debug,
{
    fn assert(&self, predicate: &Predicate, part: &Part) {
        assert!(self.result, "{}", self.message(predicate, part))
    }

    fn message(&self, predicate: &Predicate, part: &Part) -> String {
        format!(
            "{} {} {:#?}. Found {:#?}.",
            part, predicate, self.right, self.left
        )
    }
}

/// Represents an assertion event that we can emit.
///
/// An [`Assertion`] is composed of a type [`AssertType`] to determine
/// what kind of assertion to run, of a [`Predicate`] for the condition
/// of the assertion and a [`Part`] under test.
pub struct Assertion {
    ty: Box<dyn AssertKind>,
    predicate: Predicate,
    part: Part,
}

impl Assertion {
    /// Creates a new [`Assertion`].
    pub fn new(ty: Box<dyn AssertKind>, predicate: Predicate, part: Part) -> Self {
        Self {
            ty,
            predicate,
            part,
        }
    }

    /// Emits an assertion by evaluating the correct assertion macro to match.
    ///
    /// This function constructs an assertion message based on the [`Predicate`],
    /// the [`Part`] under test and the [`AssertType`], which will be used in
    /// case the assertion fails.
    ///
    /// [`emit_multi_types()`]: [`Self::emit_multi_types()`]
    pub fn emit(&self) {
        self.ty.assert(&self.predicate, &self.part)
    }
}
