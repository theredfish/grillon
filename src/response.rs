//! The `response` module provides everything to implement custom responses that can
//! be asserted with [`Assert`].
//!
//! [`Grillon`] provides a default implementation of [`Response`] with [`Hyper`](https://github.com/hyperium/hyper).
//!
//! [`Assert`]: crate::Assert
//! [`Grillon`]: crate::Grillon
use futures::{future::LocalBoxFuture, FutureExt};
use http::{HeaderMap, StatusCode};
use reqwest::Response as ReqwestResponse;
use serde_json::Value;

/// A generic http response representation with
/// convenience methods for subsequent assertions
/// with [`Assert`].
///
/// [`Assert`]: crate::Assert
pub trait Response {
    /// Returns the http status code.
    fn status(&self) -> StatusCode;
    /// Returns a future with the response json body.
    fn json<'a>(self) -> LocalBoxFuture<'a, Option<Value>>;
    /// Returns the response headers.
    fn headers(&self) -> HeaderMap;
}

impl Response for ReqwestResponse {
    fn status(&self) -> StatusCode {
        self.status()
    }

    fn json<'a>(self) -> LocalBoxFuture<'a, Option<Value>> {
        async move {
            if let Ok(bytes) = self.bytes().await {
                if bytes.is_empty() {
                    return None;
                }
                let json: Value = serde_json::from_slice(&bytes).expect("Failed to decode json");

                Some(json)
            } else {
                None
            }
        }
        .boxed_local()
    }

    fn headers(&self) -> HeaderMap {
        self.headers().clone()
    }
}
