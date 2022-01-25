//! The `response` module provides everything to implement custom responses that can
//! be asserted with [`Assert`].
//!
//! [`Grillon`] provides a default implementation of [`Response`] with [`Hyper`](https://github.com/hyperium/hyper).
//!
//! [`Assert`]: crate::Assert
//! [`Grillon`]: crate::Grillon
use futures::{future::LocalBoxFuture, FutureExt};
use hyper::{
    body::{Body, Buf},
    header::HeaderMap,
    http::{response::Response as HyperResponse, StatusCode},
};
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

impl Response for HyperResponse<Body> {
    fn status(&self) -> StatusCode {
        self.status()
    }

    fn json<'a>(self) -> LocalBoxFuture<'a, Option<Value>> {
        let (_, body) = self.into_parts();
        let body = hyper::body::aggregate(body);

        async {
            let body = body.await.expect("Valid buffer");
            if !body.has_remaining() {
                return None;
            }

            let json: Value =
                serde_json::from_reader(body.reader()).expect("Failed to decode json");

            Some(json)
        }
        .boxed_local()
    }

    fn headers(&self) -> HeaderMap {
        self.headers().clone()
    }
}
