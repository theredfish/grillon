use crate::{
    assert::{AssertBool, AssertEq, AssertNe, Assertion},
    dsl::expression::Predicate::{self, Is, IsNot},
    dsl::part::Part,
    header::{HeaderMap, HeaderName, HeaderValue},
};

type HeadersVec = Vec<(HeaderName, HeaderValue)>;

/// Http headers DSL to assert the headers of a response.
pub trait HeadersDsl<T> {
    /// Asserts the headers are strictly equal to the provided ones.
    fn is(&self, actual: T) -> Assertion;
    /// Asserts the headers are strictly not equal to the provided ones.
    fn is_not(&self, actual: T) -> Assertion;
    /// Asserts the headers contain a specific header by key - value.
    fn contains(&self, actual: T) -> Assertion;
    /// Asserts the headers does not contain a specific header by key - value.
    fn does_not_contain(&self, actual: T) -> Assertion;
    /// Evaluates the headers assertion to run based on the [`Predicate`].
    fn eval(&self, actual: T, predicate: Predicate) -> Assertion {
        match predicate {
            Predicate::Is => self.is(actual),
            Predicate::IsNot => self.is_not(actual),
            Predicate::Contains => self.contains(actual),
            Predicate::DoesNotContain => self.does_not_contain(actual),
            _ => unimplemented!("Invalid predicate for the header DSL : {predicate}"),
        }
    }
}

impl HeadersDsl<HeaderMap> for HeaderMap {
    fn is(&self, actual: HeaderMap) -> Assertion {
        let ty = AssertEq {
            left: actual,
            right: self.clone(),
        };

        Assertion::new(Box::new(ty), Is, Part::Headers)
    }

    fn is_not(&self, actual: HeaderMap) -> Assertion {
        let ty = AssertNe {
            left: actual,
            right: self.clone(),
        };

        Assertion::new(Box::new(ty), IsNot, Part::Headers)
    }

    fn contains(&self, actual: HeaderMap) -> Assertion {
        let result = actual.contains_inner(self);
        let ty = AssertBool {
            left: actual,
            right: self.clone(),
            result,
        };

        Assertion::new(Box::new(ty), Predicate::Contains, Part::Headers)
    }

    fn does_not_contain(&self, actual: HeaderMap) -> Assertion {
        let result = actual.does_not_contain_inner(self);
        let ty = AssertBool {
            left: actual,
            right: self.clone(),
            result,
        };

        Assertion::new(Box::new(ty), Predicate::DoesNotContain, Part::Headers)
    }
}

impl HeadersDsl<HeaderMap> for HeadersVec {
    fn is(&self, actual: HeaderMap) -> Assertion {
        let header_map = HeaderMap::from_iter(self.clone());
        let ty = AssertEq {
            left: actual,
            right: header_map,
        };

        Assertion::new(Box::new(ty), Is, Part::Headers)
    }

    fn is_not(&self, actual: HeaderMap) -> Assertion {
        let header_map = HeaderMap::from_iter(self.clone());
        let ty = AssertNe {
            left: actual,
            right: header_map,
        };

        Assertion::new(Box::new(ty), IsNot, Part::Headers)
    }

    fn contains(&self, actual: HeaderMap) -> Assertion {
        let result = actual.contains_inner(self);
        let ty = AssertBool {
            left: actual,
            right: self.clone(),
            result,
        };

        Assertion::new(Box::new(ty), Predicate::Contains, Part::Headers)
    }

    fn does_not_contain(&self, actual: HeaderMap) -> Assertion {
        let result = actual.does_not_contain_inner(self);
        let ty = AssertBool {
            left: actual,
            right: self.clone(),
            result,
        };

        Assertion::new(Box::new(ty), Predicate::Contains, Part::Headers)
    }
}

trait InnerCheck<T> {
    fn contains_inner(&self, other: &T) -> bool;
    fn does_not_contain_inner(&self, other: &T) -> bool;
}

macro_rules! inner_check {
    ($t:ty, $u:ty) => {
        impl InnerCheck<$u> for $t {
            fn contains_inner(&self, other: &$u) -> bool {
                let actual_empty = self.is_empty();
                let other_empty = other.is_empty();

                // We first check for difference by emptiness. If the comparand
                // isn't empty, but the actual iterator is, we can directly
                // return false.
                // But in case the comparand is empty, we accept
                // it as a valid input, it will not enter the for loop.
                // When `other` is empty we consider the end-user doesn't want
                // to check any element, thus we fallback to `true`.
                if !other_empty && actual_empty {
                    return false;
                }

                for (key, expected_val) in other {
                    match self.get(key) {
                        Some(val) if val.eq(&expected_val) => continue,
                        _ => {
                            return false;
                        }
                    }
                }

                true
            }

            fn does_not_contain_inner(&self, other: &$u) -> bool {
                for (key, absent_val) in other {
                    match self.get(key) {
                        Some(val) if val.eq(absent_val) => {
                            return false;
                        }
                        _ => continue,
                    }
                }

                true
            }
        }
    };
}

inner_check!(HeaderMap, HeadersVec);
inner_check!(HeaderMap, HeaderMap);
