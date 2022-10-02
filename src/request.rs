//! The `request` module provides everything to build http requests
//! for endpoints under tests.
//!
//! Currently powered by the [`Hyper`](https://github.com/hyperium/hyper) HTTP client.
use std::time::Instant;

use crate::{assert::Assert, grillon::LogSettings};
use hyper::{
    body::Body,
    client::HttpConnector,
    header::{HeaderMap, HeaderName, HeaderValue},
    http::request::Request as HyperRequest,
    Client, Method, Uri,
};
use serde_json::Value;

/// List of methods where there is no associated body.
const METHODS_NO_BODY: &[Method] = &[
    Method::CONNECT,
    Method::HEAD,
    Method::GET,
    Method::OPTIONS,
    Method::TRACE,
];

/// A generic http request headers representation.
///
/// [`Grillon`] allows the use of different types
/// to represent headers that are convertible to an [`HeaderMap`].
///
/// [`Grillon`]: crate::Grillon
pub trait RequestHeaders {
    /// Converts http request headers to an [`HeaderMap`].
    /// Any type implementing this trait's function can be
    /// passed in [`Request::headers()`].
    fn to_header_map(&self) -> HeaderMap;
}

impl RequestHeaders for Vec<(HeaderName, HeaderValue)> {
    fn to_header_map(&self) -> HeaderMap {
        let mut map = HeaderMap::new();

        for (key, value) in self {
            map.append(key, value.clone());
        }

        map
    }
}

impl RequestHeaders for HeaderMap {
    fn to_header_map(&self) -> HeaderMap {
        self.clone()
    }
}

/// Represents an outgoing http request.
///
/// Can be executed with [`Request::assert()`].
pub struct Request<'c> {
    /// The http request method.
    pub method: Method,
    /// The http request uri.
    pub uri: Uri,
    /// The http request headers.
    pub headers: HeaderMap,
    /// The http request payload.
    pub payload: Option<Body>,
    /// The client used for this outgoing request.
    pub client: &'c Client<HttpConnector>,
    /// The log settings that will be used to output test results
    /// when asserting the http response.
    pub log_settings: &'c LogSettings,
}

impl Request<'_> {
    /// Sets the headers to the [`Request`].
    ///
    /// # Example
    ///
    /// ```rust
    /// # use grillon::{Grillon, Result, header};
    /// # fn run() -> Result<()> {
    /// Grillon::new("http://jsonplaceholder.typicode.com")?
    ///     .delete("users/1")
    ///     .headers(vec![(
    ///         header::CONTENT_TYPE,
    ///         header::HeaderValue::from_static("application/json"),
    ///     )]);
    /// # Ok(())
    /// # }
    /// ```
    pub fn headers<H: RequestHeaders>(mut self, headers: H) -> Self {
        self.headers = headers.to_header_map();

        self
    }

    /// Sets the body to the [`Request`].
    ///
    /// # Example
    ///
    /// ```rust
    /// # use grillon::{Grillon, Result, header, json};
    /// # fn run() -> Result<()> {
    /// Grillon::new("http://jsonplaceholder.typicode.com")?
    ///     .post("users")
    ///     .payload(json!({
    ///         "name": "Isaac",
    ///      }));
    /// # Ok(())
    /// # }
    /// ```
    pub fn payload(mut self, json: Value) -> Self {
        // TODO: See to manage this as an error to collect. To avoid confusion
        // for the user we warn him without failing since it might be intended.
        // We can maybe find a better way to manage this case.
        if METHODS_NO_BODY.contains(&self.method) {
            println!(
                "{} does not support HTTP body. No payload will be sent.",
                self.method
            );

            return self;
        }

        self.payload = Some(Body::from(json.to_string()));

        self
    }

    /// Sends the http request and creates an instance of [`Assert`] with the http response.
    ///
    /// This function consumes the [`Request`].
    ///
    /// # Example
    ///
    /// ```rust
    /// # use grillon::{Grillon, Result};
    /// # async fn run() -> Result<()> {
    /// Grillon::new("http://jsonplaceholder.typicode.com")?
    ///     .get("users")
    ///     .assert()
    ///     .await;
    /// # Ok(())
    /// # }
    /// ```
    pub async fn assert(self) -> Assert {
        let mut req = HyperRequest::new(self.payload.unwrap_or_else(Body::empty));
        *req.method_mut() = self.method;
        *req.headers_mut() = self.headers;
        *req.uri_mut() = self.uri;

        let now = Instant::now();
        let response = self
            .client
            .request(req)
            .await
            // TODO : replace this expect by an assertion on the response itself.
            .expect("Failed to send http request");

        // Due to serde limitations with 128bits we need to cast u128 to u64
        // with the risk to lose precision. However should be acceptable since
        // the api of Duration::from_millis() accepts a u64 value.
        //
        // See https://github.com/serde-rs/serde/issues/1717
        // See https://github.com/serde-rs/serde/issues/1183
        let response_time_ms = now.elapsed().as_millis() as u64;

        Assert::new(response, response_time_ms, self.log_settings.clone()).await
    }
}
