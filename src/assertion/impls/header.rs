use crate::assertion::traits::{Container, Equality};
use crate::assertion::types::{HeaderTupleVec, Headers};
use crate::assertion::{Assertion, AssertionResult, Hand};
use crate::dsl::{Part, Predicate};
use crate::header::HeaderMap;

/// Converts [`HeaderTupleVec`] into [`Headers`].
pub fn from_header_tuple_vec(header_tuple_vec: &HeaderTupleVec) -> Headers {
    header_tuple_vec
        .iter()
        .map(|(header_name, header_value)| {
            (
                header_name.to_string(),
                header_value.to_str().unwrap().to_string(),
            )
        })
        .collect()
}

/// Converts [`HeaderMap`] into [`Headers`].
pub fn from_header_map(header_map: &HeaderMap) -> Headers {
    header_map
        .iter()
        .map(|(header_name, header_value)| {
            (
                header_name.to_string(),
                header_value.to_str().unwrap().to_string(),
            )
        })
        .collect()
}

/// Matches macro signature to convert [`Headers`].
fn from_headers(headers: &Headers) -> Headers {
    headers.to_owned()
}

/// Compares equality between two [`Headers`] without considering the order of
/// the headers. As long as their keys and values match they are considered
/// equal.
/// This function sorts and deduplicate tuples of headers before testing the
/// equality.
fn headers_equal_any_order(left: &mut Headers, right: &mut Headers) -> bool {
    left.sort();
    left.dedup();
    right.sort();
    right.dedup();

    left == right
}

impl Equality<HeaderMap> for HeaderMap {
    type Assertion = Assertion<Headers>;

    fn is_eq(&self, rhs: &HeaderMap) -> Self::Assertion {
        let mut lhs: Headers = from_header_map(self);
        let mut rhs: Headers = from_header_map(rhs);
        let result = headers_equal_any_order(&mut lhs, &mut rhs);

        Assertion {
            predicate: Predicate::Is,
            part: Part::Headers,
            left: Hand::Left(lhs.clone()),
            right: Hand::Right(rhs.clone()),
            result: result.into(),
        }
    }

    fn is_ne(&self, rhs: &HeaderMap) -> Self::Assertion {
        let mut lhs: Headers = from_header_map(self);
        let mut rhs: Headers = from_header_map(rhs);
        let result = !headers_equal_any_order(&mut lhs, &mut rhs);

        Assertion {
            predicate: Predicate::IsNot,
            part: Part::Headers,
            left: Hand::Left(lhs.clone()),
            right: Hand::Right(rhs.clone()),
            result: result.into(),
        }
    }
}

impl Equality<HeaderTupleVec> for HeaderMap {
    type Assertion = Assertion<Headers>;

    fn is_eq(&self, rhs: &HeaderTupleVec) -> Self::Assertion {
        let mut lhs: Headers = from_header_map(self);
        let mut rhs: Headers = from_header_tuple_vec(rhs);
        let result = headers_equal_any_order(&mut lhs, &mut rhs);

        Assertion {
            predicate: Predicate::Is,
            part: Part::Headers,
            left: Hand::Left(lhs.clone()),
            right: Hand::Right(rhs.clone()),
            result: result.into(),
        }
    }

    fn is_ne(&self, rhs: &HeaderTupleVec) -> Self::Assertion {
        let mut lhs: Headers = from_header_map(self);
        let mut rhs: Headers = from_header_tuple_vec(rhs);
        let result = !headers_equal_any_order(&mut lhs, &mut rhs);

        Assertion {
            predicate: Predicate::IsNot,
            part: Part::Headers,
            left: Hand::Left(lhs.clone()),
            right: Hand::Right(rhs.clone()),
            result: result.into(),
        }
    }
}

impl Equality<Headers> for HeaderMap {
    type Assertion = Assertion<Headers>;

    fn is_eq(&self, rhs: &Headers) -> Self::Assertion {
        let mut lhs: Headers = from_header_map(self);
        let mut rhs = rhs.clone();
        let result = headers_equal_any_order(&mut lhs, &mut rhs);

        Assertion {
            predicate: Predicate::Is,
            part: Part::Headers,
            left: Hand::Left(lhs.clone()),
            right: Hand::Right(rhs.clone()),
            result: result.into(),
        }
    }

    fn is_ne(&self, rhs: &Headers) -> Self::Assertion {
        let mut lhs: Headers = from_header_map(self);
        let mut rhs = rhs.clone();
        let result = !headers_equal_any_order(&mut lhs, &mut rhs);

        Assertion {
            predicate: Predicate::IsNot,
            part: Part::Headers,
            left: Hand::Left(lhs.clone()),
            right: Hand::Right(rhs.clone()),
            result: result.into(),
        }
    }
}

macro_rules! impl_container {
    ($lhs:ty, $rhs:ty, $from:expr) => {
        impl Container<$rhs> for $lhs {
            type Assertion = Assertion<Headers>;

            fn has(&self, rhs: &$rhs) -> Self::Assertion {
                let lhs_headers: Headers = from_header_map(self);
                let rhs_headers: Headers = $from(rhs);

                let lhs_empty = lhs_headers.is_empty();
                let rhs_empty = rhs_headers.is_empty();

                // We first check for difference by emptiness. If the comparand
                // isn't empty, but the actual iterator is, we can directly
                // return false.
                // But in case the comparand is empty, we accept
                // it as a valid input, it will not enter the for loop.
                // When `other` is empty we consider the end-user doesn't want
                // to check any element, thus we fallback to `true`.
                if !rhs_empty && lhs_empty {
                    return Assertion {
                        predicate: Predicate::Contains,
                        part: Part::Headers,
                        left: Hand::Left(lhs_headers),
                        right: Hand::Right(rhs_headers),
                        result: AssertionResult::Failed,
                    };
                }

                for (key, expected_val) in rhs {
                    match self.get(key) {
                        Some(val) if val.eq(&expected_val) => continue,
                        _ => {
                            return Assertion {
                                predicate: Predicate::Contains,
                                part: Part::Headers,
                                left: Hand::Left(lhs_headers),
                                right: Hand::Right(rhs_headers),
                                result: AssertionResult::Failed,
                            };
                        }
                    }
                }

                Assertion {
                    predicate: Predicate::Contains,
                    part: Part::Headers,
                    left: Hand::Left(lhs_headers),
                    right: Hand::Right(rhs_headers),
                    result: AssertionResult::Passed,
                }
            }

            fn has_not(&self, rhs: &$rhs) -> Self::Assertion {
                let lhs_headers: Headers = from_header_map(self);
                let rhs_headers: Headers = $from(rhs);

                for (key, absent_val) in rhs {
                    match self.get(key) {
                        Some(val) if val.eq(absent_val) => {
                            return Assertion {
                                predicate: Predicate::DoesNotContain,
                                part: Part::Headers,
                                left: Hand::Left(lhs_headers),
                                right: Hand::Right(rhs_headers),
                                result: AssertionResult::Failed,
                            };
                        }
                        _ => continue,
                    }
                }

                Assertion {
                    predicate: Predicate::DoesNotContain,
                    part: Part::Headers,
                    left: Hand::Left(lhs_headers),
                    right: Hand::Right(rhs_headers),
                    result: AssertionResult::Passed,
                }
            }
        }
    };
}

impl_container!(HeaderMap, HeaderMap, from_header_map);
impl_container!(HeaderMap, HeaderTupleVec, from_header_tuple_vec);
impl_container!(HeaderMap, Headers, from_headers);

#[cfg(test)]
mod tests {
    use super::{from_header_map, HeaderTupleVec, Headers};
    use crate::{
        assertion::{
            impls::header::{from_header_tuple_vec, headers_equal_any_order},
            traits::{Container, Equality},
        },
        header::{HeaderMap, HeaderValue, CONTENT_LENGTH, CONTENT_TYPE, DATE},
    };

    fn header_map_stub() -> HeaderMap {
        let mut header_map = HeaderMap::new();
        header_map.insert(CONTENT_TYPE, HeaderValue::from_static("application/json"));
        header_map.insert(CONTENT_LENGTH, HeaderValue::from_static("23"));
        header_map.insert(DATE, HeaderValue::from_static("today"));

        header_map
    }

    fn header_tuple_vec_stub() -> HeaderTupleVec {
        vec![
            (CONTENT_TYPE, HeaderValue::from_static("application/json")),
            (CONTENT_LENGTH, HeaderValue::from_static("23")),
            (DATE, HeaderValue::from_static("today")),
        ]
    }

    fn headers_stub() -> Headers {
        vec![
            ("content-type".to_string(), "application/json".to_string()),
            ("content-length".to_string(), "23".to_string()),
            ("date".to_string(), "today".to_string()),
        ]
    }

    #[test]
    fn it_should_convert_from_header_map() {
        let headers = from_header_map(&header_map_stub());
        let expected_headers: Headers = vec![
            (CONTENT_TYPE.to_string(), "application/json".to_string()),
            (CONTENT_LENGTH.to_string(), "23".to_string()),
            (DATE.to_string(), "today".to_string()),
        ];

        assert_eq!(
            headers, expected_headers,
            "{headers:#?} isn't equals to the expected header {expected_headers:#?}"
        );
    }

    #[test]
    fn it_should_convert_from_header_tuple_vec() {
        let headers = from_header_tuple_vec(&header_tuple_vec_stub());
        let expected_headers: Headers = vec![
            (CONTENT_TYPE.to_string(), "application/json".to_string()),
            (CONTENT_LENGTH.to_string(), "23".to_string()),
            (DATE.to_string(), "today".to_string()),
        ];

        assert_eq!(
            headers, expected_headers,
            "{headers:#?} isn't equals to the expected header {expected_headers:#?}"
        );
    }

    #[test]
    fn headers_should_be_equal_in_any_order() {
        let mut permutation1: Headers = vec![
            (CONTENT_TYPE.to_string(), "application/json".to_string()),
            (DATE.to_string(), "today".to_string()),
            (CONTENT_LENGTH.to_string(), "23".to_string()),
        ];
        let mut permutation2: Headers = vec![
            (CONTENT_LENGTH.to_string(), "23".to_string()),
            (CONTENT_TYPE.to_string(), "application/json".to_string()),
            (DATE.to_string(), "today".to_string()),
        ];
        let mut permutation3: Headers = vec![
            (CONTENT_TYPE.to_string(), "application/json".to_string()),
            (DATE.to_string(), "today".to_string()),
            (CONTENT_LENGTH.to_string(), "23".to_string()),
        ];
        let mut permutation4: Headers = vec![
            (DATE.to_string(), "today".to_string()),
            (CONTENT_TYPE.to_string(), "application/json".to_string()),
            (CONTENT_LENGTH.to_string(), "23".to_string()),
        ];

        assert!(headers_equal_any_order(
            &mut headers_stub(),
            &mut permutation1
        ));

        assert!(headers_equal_any_order(
            &mut headers_stub(),
            &mut permutation2
        ));

        assert!(headers_equal_any_order(
            &mut headers_stub(),
            &mut permutation3
        ));

        assert!(headers_equal_any_order(
            &mut headers_stub(),
            &mut permutation4
        ));

        // same order
        assert!(headers_equal_any_order(
            &mut headers_stub(),
            &mut headers_stub()
        ));
    }

    #[test]
    fn impl_is_eq_header_map() {
        let assertion = header_map_stub().is_eq(&header_map_stub());

        assert!(assertion.passed(), "{}", assertion.message());
    }

    #[test]
    fn impl_is_ne_header_map() {
        let mut header_map = header_map_stub();
        header_map.insert(
            CONTENT_TYPE,
            HeaderValue::from_static("text/html; charset=utf-8"),
        );

        let assertion = header_map_stub().is_ne(&header_map);

        assert!(assertion.passed(), "{}", assertion.message());
    }

    #[test]
    fn impl_is_eq_header_tuple_vec() {
        let assertion = header_map_stub().is_eq(&header_tuple_vec_stub());

        assert!(assertion.passed(), "{}", assertion.message());
    }

    #[test]
    fn impl_is_ne_header_tuple_vec() {
        let mut header_tuple_vec = header_tuple_vec_stub();
        header_tuple_vec[0] = (
            CONTENT_TYPE,
            HeaderValue::from_static("text/html; charset=utf-8"),
        );

        let assertion = header_map_stub().is_ne(&header_tuple_vec);

        assert!(assertion.passed(), "{}", assertion.message());
    }

    #[test]
    fn impl_is_eq_headers() {
        let assertion = header_map_stub().is_eq(&headers_stub());

        assert!(assertion.passed(), "{}", assertion.message());
    }

    #[test]
    fn impl_is_ne_headers() {
        let mut headers = headers_stub();
        headers[0] = (
            "content-type".to_string(),
            "text/html; charset=utf-8".to_string(),
        );

        let assertion = header_map_stub().is_ne(&headers);

        assert!(assertion.passed(), "{}", assertion.message());
    }

    #[test]
    fn impl_has_header_map() {
        let mut header_map_one_item = HeaderMap::new();
        let mut header_map_two_items = HeaderMap::new();
        header_map_one_item.insert(CONTENT_TYPE, HeaderValue::from_static("application/json"));
        header_map_two_items.insert(CONTENT_TYPE, HeaderValue::from_static("application/json"));
        header_map_two_items.insert(DATE, HeaderValue::from_static("today"));

        let assertion = header_map_stub().has(&header_map_one_item);
        assert!(assertion.passed(), "{}", assertion.message());

        let assertion = header_map_stub().has(&header_map_two_items);
        assert!(assertion.passed(), "{}", assertion.message());
    }

    #[test]
    fn impl_has_not_header_map() {
        let mut header_map_one_item = HeaderMap::new();
        let mut header_map_two_items = HeaderMap::new();
        header_map_one_item.insert(
            CONTENT_TYPE,
            HeaderValue::from_static("text/html; charset=utf-8"),
        );
        header_map_two_items.insert(
            CONTENT_TYPE,
            HeaderValue::from_static("text/html; charset=utf-8"),
        );
        header_map_two_items.insert(DATE, HeaderValue::from_static("tomorrow"));

        let assertion = header_map_stub().has_not(&header_map_one_item);
        assert!(assertion.passed(), "{}", assertion.message());

        let assertion = header_map_stub().has_not(&header_map_two_items);
        assert!(assertion.passed(), "{}", assertion.message());
    }

    #[test]
    fn impl_has_not_header_map_fails_if_at_least_one_item_matches() {
        let mut header_map = HeaderMap::new();
        header_map.insert(
            CONTENT_TYPE,
            HeaderValue::from_static("text/html; charset=utf-8"),
        );
        // this item matches
        header_map.insert(DATE, HeaderValue::from_static("today"));

        let assertion = header_map_stub().has_not(&header_map);
        assert!(assertion.failed(), "{}", assertion.message());
    }

    #[test]
    fn impl_has_header_tuple_vec() {
        let header_tuple_vec_one_item =
            vec![(CONTENT_TYPE, HeaderValue::from_static("application/json"))];
        let header_tuple_vec_two_items = vec![
            (CONTENT_TYPE, HeaderValue::from_static("application/json")),
            (DATE, HeaderValue::from_static("today")),
        ];

        let assertion = header_map_stub().has(&header_tuple_vec_one_item);
        assert!(assertion.passed(), "{}", assertion.message());

        let assertion = header_map_stub().has(&header_tuple_vec_two_items);
        assert!(assertion.passed(), "{}", assertion.message());
    }

    #[test]
    fn impl_has_not_header_tuple_vec() {
        let header_tuple_vec_one_item = vec![(
            CONTENT_TYPE,
            HeaderValue::from_static("text/html; charset=utf-8"),
        )];
        let header_tuple_vec_two_items = vec![
            (
                CONTENT_TYPE,
                HeaderValue::from_static("text/html; charset=utf-8"),
            ),
            (DATE, HeaderValue::from_static("tomorrow")),
        ];

        let assertion = header_map_stub().has_not(&header_tuple_vec_one_item);
        assert!(assertion.passed(), "{}", assertion.message());

        let assertion = header_map_stub().has_not(&header_tuple_vec_two_items);
        assert!(assertion.passed(), "{}", assertion.message());
    }

    #[test]
    fn impl_has_headers() {
        let headers_one_item = vec![(CONTENT_TYPE.to_string(), "application/json".to_string())];
        let headers_two_items = vec![
            (CONTENT_TYPE.to_string(), "application/json".to_string()),
            (DATE.to_string(), "today".to_string()),
        ];

        let assertion = header_map_stub().has(&headers_one_item);
        assert!(assertion.passed(), "{}", assertion.message());

        let assertion = header_map_stub().has(&headers_two_items);
        assert!(assertion.passed(), "{}", assertion.message());
    }

    #[test]
    fn impl_has_not_headers() {
        let headers_one_item = vec![(
            CONTENT_TYPE.to_string(),
            "text/html; charset=utf-8".to_string(),
        )];
        let headers_two_items = vec![
            (
                CONTENT_TYPE.to_string(),
                "text/html; charset=utf-8".to_string(),
            ),
            (DATE.to_string(), "tomorrow".to_string()),
        ];

        let assertion = header_map_stub().has_not(&headers_one_item);
        assert!(assertion.passed(), "{}", assertion.message());

        let assertion = header_map_stub().has_not(&headers_two_items);
        assert!(assertion.passed(), "{}", assertion.message());
    }

    mod serialization {
        use super::*;
        use serde_json::json;

        #[test]
        fn it_serializes_headers_should_be() {
            // Specific order resulting from the sort
            let headers = json!([
                ["content-length", "23"],
                ["content-type", "application/json"],
                ["date", "today"]
            ]);
            let expected_json = json!({
                "part": "headers",
                "predicate": "should be",
                "left": headers,
                "right": headers,
                "result": "passed"
            });

            let assertion = header_map_stub().is_eq(&header_map_stub());

            assert_eq!(
                json!(assertion),
                expected_json,
                "Serialized assertion is not equals to the expected json",
            );
        }

        #[test]
        fn it_serializes_headers_should_not_be() {
            let mut header_map = header_map_stub();
            header_map.insert(
                CONTENT_TYPE,
                HeaderValue::from_static("text/html; charset=utf-8"),
            );

            // Specific order resulting from the sort
            let lhs_headers = json!([
                ["content-length", "23"],
                ["content-type", "application/json"],
                ["date", "today"]
            ]);
            let rhs_headers = json!([
                ["content-length", "23"],
                ["content-type", "text/html; charset=utf-8"],
                ["date", "today"]
            ]);
            let expected_json = json!({
                "part": "headers",
                "predicate": "should not be",
                "left": lhs_headers,
                "right": rhs_headers,
                "result": "passed"
            });

            let assertion = header_map_stub().is_ne(&header_map);

            assert_eq!(
                json!(assertion),
                expected_json,
                "Serialized assertion is not equals to the expected json",
            );
        }

        #[test]
        fn it_serializes_headers_has_one_header() {
            let headers_rhs = vec![("content-type".to_string(), "application/json".to_string())];
            let expected_json = json!({
                "part": "headers",
                "predicate": "should contain",
                "left": json!(headers_stub()),
                "right": headers_rhs,
                "result": "passed"
            });

            let assertion = header_map_stub().has(&headers_rhs);

            assert_eq!(
                json!(assertion),
                expected_json,
                "Serialized assertion is not equals to the expected json",
            );
        }

        #[test]
        fn it_serializes_headers_has_several_headers() {
            let headers_rhs = vec![
                ("content-type".to_string(), "application/json".to_string()),
                ("date".to_string(), "today".to_string()),
            ];
            let expected_json = json!({
                "part": "headers",
                "predicate": "should contain",
                "left": json!(headers_stub()),
                "right": headers_rhs,
                "result": "passed"
            });

            let assertion = header_map_stub().has(&headers_rhs);

            assert_eq!(
                json!(assertion),
                expected_json,
                "Serialized assertion is not equals to the expected json",
            );
        }

        #[test]
        fn it_serializes_headers_has_not_one_header() {
            let headers_rhs = vec![(
                "content-type".to_string(),
                "text/html; charset=utf-8".to_string(),
            )];
            let expected_json = json!({
                "part": "headers",
                "predicate": "should not contain",
                "left": json!(headers_stub()),
                "right": headers_rhs,
                "result": "passed"
            });

            let assertion = header_map_stub().has_not(&headers_rhs);

            assert_eq!(
                json!(assertion),
                expected_json,
                "Serialized assertion is not equals to the expected json",
            );
        }

        #[test]
        fn it_serializes_headers_has_not_several_headers() {
            let headers_rhs = vec![
                (
                    "content-type".to_string(),
                    "text/html; charset=utf-8".to_string(),
                ),
                ("date".to_string(), "tomorrow".to_string()),
            ];
            let expected_json = json!({
                "part": "headers",
                "predicate": "should not contain",
                "left": json!(headers_stub()),
                "right": headers_rhs,
                "result": "passed"
            });

            let assertion = header_map_stub().has_not(&headers_rhs);

            assert_eq!(
                json!(assertion),
                expected_json,
                "Serialized assertion is not equals to the expected json",
            );
        }
    }
}
