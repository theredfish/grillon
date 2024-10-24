//! The `request` module provides everything to build http requests
//! for endpoints under tests.
//!
//! Currently powered by the [`Reqwest`](https://github.com/seanmonstar/reqwest) HTTP client.
use std::{str::FromStr, time::Instant};

use crate::assertion::{Assertion, AssertionResult, Hand, UnprocessableReason};
use crate::dsl::{Part, Predicate};
use crate::error::Result;
use crate::{assert::Assert, grillon::LogSettings};
use http::{HeaderMap, HeaderName, HeaderValue, Method};
use reqwest::{Body, Client};
use serde_json::Value;
use url::Url;

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
    fn to_header_map(&self) -> Result<HeaderMap>;
}

impl RequestHeaders for Vec<(HeaderName, HeaderValue)> {
    fn to_header_map(&self) -> Result<HeaderMap> {
        let mut map = HeaderMap::new();

        for (key, value) in self {
            map.append(key, value.clone());
        }

        Ok(map)
    }
}

impl RequestHeaders for Vec<(&str, &str)> {
    fn to_header_map(&self) -> Result<HeaderMap> {
        let mut map = HeaderMap::new();

        for (key, value) in self {
            map.append(HeaderName::from_str(key)?, HeaderValue::from_str(value)?);
        }

        Ok(map)
    }
}

impl RequestHeaders for HeaderMap {
    fn to_header_map(&self) -> Result<HeaderMap> {
        Ok(self.clone())
    }
}

/// Represents an outgoing http request.
///
/// Can be executed with [`Request::assert()`].
pub struct Request<'c> {
    /// The http request method.
    pub method: Method,
    /// The http request url.
    pub url: Url,
    /// The http request headers.
    pub headers: Result<HeaderMap>,
    /// The http request payload.
    pub payload: Option<Body>,
    /// The client used for this outgoing request.
    pub client: &'c Client,
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
    /// Grillon::new("https://jsonplaceholder.typicode.com")?
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
    /// Grillon::new("https://jsonplaceholder.typicode.com")?
    ///     .post("users")
    ///     .payload(json!({
    ///         "name": "Isaac",
    ///      }));
    /// # Ok(())
    /// # }
    /// ```
    pub fn payload(mut self, json: Value) -> Self {
        // TODO: See to manage this as an error to collect. To avoid confusion
        // for users we warn them without failing since it might be intended.
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
    /// Grillon::new("https://jsonplaceholder.typicode.com")?
    ///     .get("users")
    ///     .assert()
    ///     .await;
    /// # Ok(())
    /// # }
    /// ```
    pub async fn assert(self) -> Assert {
        let headers = match self.headers {
            Ok(headers) => headers,
            Err(err) => {
                let assertion = Assertion {
                    part: Part::Headers,
                    predicate: Predicate::NoPredicate,
                    left: Hand::Empty::<Value>,
                    right: Hand::Empty,
                    result: AssertionResult::Unprocessable(
                        UnprocessableReason::InvalidHttpRequestHeaders(err.to_string()),
                    ),
                };

                assertion.assert(self.log_settings);

                return Assert::new(None::<reqwest::Response>, None, self.log_settings.clone())
                    .await;
            }
        };

        let req = self
            .client
            .request(self.method, self.url)
            .body(self.payload.unwrap_or_default())
            .headers(headers);

        let now = Instant::now();
        let response = match req.send().await {
            Ok(response) => response,
            Err(err) => {
                let assertion = Assertion {
                    part: Part::NoPart,
                    predicate: Predicate::NoPredicate,
                    left: Hand::Empty::<Value>,
                    right: Hand::Empty,
                    result: AssertionResult::Unprocessable(
                        UnprocessableReason::HttpRequestFailure(err.to_string()),
                    ),
                };

                assertion.assert(self.log_settings);

                return Assert::new(None::<reqwest::Response>, None, self.log_settings.clone())
                    .await;
            }
        };

        // Due to serde limitations with 128bits we need to cast u128 to u64
        // with the risk to lose precision. However should be acceptable since
        // the api of Duration::from_millis() accepts a u64 value.
        //
        // See https://github.com/serde-rs/serde/issues/1717
        // See https://github.com/serde-rs/serde/issues/1183
        let response_time_ms = now.elapsed().as_millis() as u64;

        Assert::new(
            Some(response),
            Some(response_time_ms),
            self.log_settings.clone(),
        )
        .await
    }
}
