use hyper::{
    header::{HeaderName, HeaderValue},
    HeaderMap,
};

pub trait HeadersMatch {
    fn matches(&self, other: &HeaderMap) -> bool;
}

pub trait HeadersExcept {
    fn except(&self, other: &HeaderMap) -> bool;
}

impl HeadersMatch for Vec<(HeaderName, HeaderValue)> {
    fn matches(&self, other: &HeaderMap) -> bool {
        for (key, expected_val) in self {
            match other.get(key) {
                // the header key doesn't exist
                None => return false,
                // the header value doesn't match
                Some(val) if val.ne(expected_val) => return false,
                _ => continue,
            }
        }

        true
    }
}

impl HeadersMatch for HeaderMap {
    fn matches(&self, other: &HeaderMap) -> bool {
        for (key, expected_val) in self {
            match other.get(key) {
                // the header key doesn't exist
                None => return false,
                // the header value doesn't match
                Some(val) if val.ne(expected_val) => return false,
                _ => continue,
            }
        }

        true
    }
}

impl HeadersExcept for Vec<(HeaderName, HeaderValue)> {
    fn except(&self, other: &HeaderMap) -> bool {
        for (key, expected_val) in self {
            match other.get(key) {
                // if the header value is present, then the test fails
                Some(val) if val.eq(expected_val) => return false,
                // if the key doesn't exist or the value isn't equals
                _ => continue,
            }
        }

        true
    }
}

impl HeadersExcept for HeaderMap {
    fn except(&self, other: &HeaderMap) -> bool {
        for (key, expected_val) in self {
            match other.get(key) {
                // if the header value is present, then the test fails
                Some(val) if val.eq(expected_val) => return false,
                // if the key doesn't exist or the value isn't equals
                _ => continue,
            }
        }

        true
    }
}
