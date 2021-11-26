use hyper::{
    header::{HeaderName, HeaderValue},
    HeaderMap,
};

pub trait HeadersExist {
    fn exist(&self, other: &HeaderMap) -> bool;
}

pub trait HeadersExcept {
    fn except(&self, other: &HeaderMap) -> bool;
}

macro_rules! exist {
    ($t:ty) => {
        impl HeadersExist for $t {
            fn exist(&self, other: &HeaderMap) -> bool {
                let actual_empty = self.is_empty();
                let other_empty = other.is_empty();

                // If one header map is empty and the other one is also empty, it matches.
                // If one header map is empty and not the other one, it doesn't match.
                // If both are not empty, we continue to compare keys and values.
                if actual_empty && other_empty {
                    return true;
                } else if actual_empty || other_empty {
                    return false;
                }

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
    };
}

macro_rules! except {
    ($t:ty) => {
        impl HeadersExcept for $t {
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
    };
}

exist!(Vec<(HeaderName, HeaderValue)>);
exist!(HeaderMap);

except!(Vec<(HeaderName, HeaderValue)>);
except!(HeaderMap);
