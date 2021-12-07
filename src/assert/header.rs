//! A collection of header matchers to use with [`Assert::headers_exist()`] and [`Assert::headers_absent()`].
//!
//! [`Assert::headers_exist()`]: crate::Assert::headers_exist
//! [`Assert::headers_absent()`]: crate::Assert::headers_absent
use hyper::{
    header::{HeaderName, HeaderValue},
    HeaderMap,
};

/// A generic matcher to check the existence of the given
/// headers in the response headers.
pub trait HeadersExistMatcher {
    /// Asserts if the headers exist in the response headers.
    fn exist(&self, response_headers: &HeaderMap);
}

/// A generic matcher to check the absence of the given
/// headers in the response headers.
pub trait HeadersAbsentMatcher {
    /// Asserts if the headers are absent from the response headers.
    fn absent(&self, response_headers: &HeaderMap);
}

macro_rules! exist {
    ($t:ty) => {
        impl HeadersExistMatcher for $t {
            fn exist(&self, response_headers: &HeaderMap) {
                let mut exist = true;
                let expected_empty = self.is_empty();
                let actual_empty = response_headers.is_empty();

                // We assert emptiness early.
                // If the expected header list is empty, there is no header to compare with,
                // in that case we consider that it matches and we will fallback to the last assertion.
                // Else if the expected header list contains at least one header but the current header list
                // is empty, we know that it doesn't match. The first assertion will fail.
                // Else we continue and we compare the headers by key/value.
                if !expected_empty && actual_empty {
                    exist = false;
                }

                assert_eq!(
                    exist,
                    true,
                    "One or more headers missing in the http response. Expected : {:#?}, Found : {:#?}",
                    self,
                    response_headers,
                );

                // Both header maps are not empty, we continue to compare keys and values.
                for (key, expected_val) in self {
                    match response_headers.get(key) {
                        // As long as we have a matching key/value for headers we continue
                        Some(val) if val.eq(expected_val) => continue,
                        // Else we break early and we consider the expected headers do not match the actual ones.
                        _ => {
                            exist = false;
                            break;
                        }
                    }
                }

                assert_eq!(
                    exist,
                    true,
                    "One or more headers missing from the http response. Expected : {:#?}, Found : {:#?}",
                    self,
                    response_headers,
                );
            }
        }
    };
}

macro_rules! absent {
    ($t:ty) => {
        impl HeadersAbsentMatcher for $t {
            fn absent(&self, response_headers: &HeaderMap) {
                let mut header_present = false;

                for (key, absent_val) in self {
                    match response_headers.get(key) {
                        // if the header value is present, then the test fails
                        Some(val) if val.eq(absent_val) => {
                            header_present = true;
                            break;
                        }
                        // if the key doesn't exist or the value isn't equals we continue to iterate
                        _ => continue,
                    }
                }

                assert_eq!(
                    header_present, false,
                    "One or more unexpected headers found in the http response : {:#?}, Response headers : {:#?}",
                    self, response_headers,
                );
            }
        }
    };
}

exist!(Vec<(HeaderName, HeaderValue)>);
exist!(HeaderMap);

absent!(Vec<(HeaderName, HeaderValue)>);
absent!(HeaderMap);
