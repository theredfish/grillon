use crate::{
    assertion::{
        traits::{Container, Equality},
        types::Headers,
        Assertion,
    },
    dsl::expression::Predicate,
    header::{HeaderMap, HeaderName, HeaderValue},
    LogSettings,
};

type HeadersVec = Vec<(HeaderName, HeaderValue)>;

/// Http headers DSL to assert the headers of a response.
pub trait HeadersDsl<T> {
    /// Asserts the headers are strictly equal to the provided ones.
    fn is(&self, actual: T) -> Assertion<Headers>;
    /// Asserts the headers are strictly not equal to the provided ones.
    fn is_not(&self, actual: T) -> Assertion<Headers>;
    /// Asserts the headers contain a specific header by key - value.
    fn contains(&self, actual: T) -> Assertion<Headers>;
    /// Asserts the headers does not contain a specific header by key - value.
    fn does_not_contain(&self, actual: T) -> Assertion<Headers>;
    /// Evaluates the headers assertion to run based on the [`Predicate`].
    fn eval(
        &self,
        actual: T,
        predicate: Predicate,
        log_settings: &LogSettings,
    ) -> Assertion<Headers> {
        match predicate {
            Predicate::Is => self.is(actual).assert(log_settings),
            Predicate::IsNot => self.is_not(actual).assert(log_settings),
            Predicate::Contains => self.contains(actual).assert(log_settings),
            Predicate::DoesNotContain => self.does_not_contain(actual).assert(log_settings),
            _ => unimplemented!("Invalid predicate for the header DSL : {predicate}"),
        }
    }
}

impl HeadersDsl<HeaderMap> for HeaderMap {
    fn is(&self, actual: HeaderMap) -> Assertion<Headers> {
        actual.is_eq(self)
    }

    fn is_not(&self, actual: HeaderMap) -> Assertion<Headers> {
        actual.is_ne(self)
    }

    fn contains(&self, actual: HeaderMap) -> Assertion<Headers> {
        actual.has(self)
    }

    fn does_not_contain(&self, actual: HeaderMap) -> Assertion<Headers> {
        actual.has_not(self)
    }
}

impl HeadersDsl<HeaderMap> for HeadersVec {
    fn is(&self, actual: HeaderMap) -> Assertion<Headers> {
        actual.is_eq(self)
    }

    fn is_not(&self, actual: HeaderMap) -> Assertion<Headers> {
        actual.is_ne(self)
    }

    fn contains(&self, actual: HeaderMap) -> Assertion<Headers> {
        actual.has(self)
    }

    fn does_not_contain(&self, actual: HeaderMap) -> Assertion<Headers> {
        actual.has_not(self)
    }
}
