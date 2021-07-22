use hyper::{
    header::{HeaderName, HeaderValue},
    HeaderMap,
};

pub trait ExpectHeaders {
    fn matches(&self, other: HeaderMap) -> bool;
}

impl ExpectHeaders for Vec<(HeaderName, HeaderValue)> {
    fn matches(&self, other: HeaderMap) -> bool {
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

impl ExpectHeaders for HeaderMap {
    fn matches(&self, other: HeaderMap) -> bool {
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
