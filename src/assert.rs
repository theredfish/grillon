//! The `assert` module provides everything to assert parts of http responses with built-in matchers.
//!
//! [`Assert`] can be used separately with your own [`Response`] implementation which makes it
//! handy if you want to use your own http client to send requests and handle responses.
//!
//! # Example of usage with `reqwest`
//!
//! ```rust
//! #[tokio::test]
//! async fn custom_response_struct() -> Result<(), grillon::Error> {
//!     use async_trait::async_trait;
//!     use grillon::{header::HeaderMap, Assert, Response, StatusCode};
//!     use serde_json::Value;
//!
//!     struct ResponseWrapper {
//!         pub response: reqwest::Response,
//!     }
//!
//!     #[async_trait(?Send)]
//!     impl Response for ResponseWrapper {
//!         fn status(&self) -> StatusCode {
//!             self.response.status()
//!         }
//!
//!         async fn json(self) -> Option<Value> {
//!             self.response.json::<Value>().await.ok()
//!         }
//!
//!         fn headers(&self) -> HeaderMap {
//!             self.response.headers().clone()
//!         }
//!     }
//!
//!     let response = reqwest::get("https://jsonplaceholder.typicode.com/users/1")
//!         .await
//!         .expect("Valid reqwest::Response");
//!     let response_wrapper = ResponseWrapper { response };
//!
//!     Assert::new(response_wrapper).await.status(is_between(200, 299));
//!
//!     Ok(())
//!  }
//! ```

use crate::assertion::{Assertion, AssertionResult, Hand, UnprocessableReason};
use crate::dsl::http::*;
use crate::dsl::json_path::{JsonPathDsl, JsonPathResult};
use crate::dsl::{Expression, Part};
use crate::grillon::LogSettings;
use crate::Response;
use http::HeaderValue;
use http::{header::AsHeaderName, HeaderMap, StatusCode};
use serde_json::Value;

/// [`Assert`] uses an internal representation of the http response to assert
/// against. If the HTTP request was successfully sent, then each field will be
/// `Some`, otherwise `None`.
#[derive(Clone)]
pub struct Assert {
    /// The http response header to assert.
    pub headers: Option<HeaderMap>,
    /// The http response status to assert.
    pub status: Option<StatusCode>,
    /// The http response json body to assert.
    pub json: Option<Option<Value>>,
    /// The http response time (in milliseconds) to assert.
    pub response_time_ms: Option<u64>,
    /// The test results output.
    pub log_settings: LogSettings,
}

impl Assert {
    /// Creates an `Assert` instance with an internal representation
    /// of the given response to assert.
    pub async fn new(
        response: Option<impl Response>,
        response_time_ms: Option<u64>,
        log_settings: LogSettings,
    ) -> Self {
        if let Some(response) = response {
            return Assert {
                headers: Some(response.headers().clone()),
                status: Some(response.status()),
                json: Some(response.json().await),
                response_time_ms,
                log_settings,
            };
        };

        Assert {
            headers: None,
            status: None,
            json: None,
            response_time_ms: None,
            log_settings,
        }
    }

    /// Extends the built-in assertions with a custom assertion.
    /// The closure gives access to the [`Assert`] instance.
    ///
    /// # Example
    ///
    /// ```rust
    /// # use grillon::{Grillon, Result, StatusCode, dsl::{is, is_between}};
    /// # async fn custom_assert() -> Result<()> {
    /// Grillon::new("https://jsonplaceholder.typicode.com")?
    ///     .get("/users")
    ///     .assert()
    ///     .await
    ///     .status(is_between(200, 299))
    ///     .assert_fn(|assert| {
    ///         assert!(!assert.headers.is_empty());
    ///         assert!(assert.status == StatusCode::CREATED);
    ///         assert!(assert.json.is_some());
    ///
    ///         println!("Json response : {:#?}", assert.json);
    ///      })
    ///      .status(is(StatusCode::CREATED));
    ///
    /// # Ok(())
    /// # }
    /// ```
    pub fn assert_fn<F>(self, func: F) -> Assert
    where
        F: for<'a> Fn(&'a Assert),
    {
        func(&self);

        self
    }

    /// Asserts the status of the response.
    pub fn status<T>(self, expr: Expression<T>) -> Assert
    where
        T: StatusCodeDsl<StatusCode>,
    {
        if let Some(status) = self.status {
            let _assertion = expr.value.eval(status, expr.predicate, &self.log_settings);
        }

        self
    }

    /// Asserts the json body of the response.
    pub fn json_body<T>(self, expr: Expression<T>) -> Assert
    where
        T: JsonBodyDsl<Value>,
    {
        if let Some(json) = &self.json {
            let actual = if let Some(body) = json.clone() {
                body
            } else {
                let assertion = Assertion {
                    part: Part::JsonPath,
                    predicate: expr.predicate,
                    left: Hand::Empty::<Value>,
                    right: Hand::Empty,
                    result: AssertionResult::Unprocessable(UnprocessableReason::MissingJsonBody),
                };
                assertion.assert(&self.log_settings);

                return self;
            };
            let _assertion = expr.value.eval(actual, expr.predicate, &self.log_settings);
        }

        self
    }

    /// Asserts the value found at the given json path.
    pub fn json_path<T>(self, path: &str, expr: Expression<T>) -> Assert
    where
        T: JsonPathDsl<Value>,
    {
        use jsonpath_rust::JsonPathQuery;

        if let Some(json) = &self.json {
            // Check for empty json body
            let json_body = if let Some(body) = json.clone() {
                body
            } else {
                let assertion = Assertion {
                    part: Part::JsonPath,
                    predicate: expr.predicate,
                    left: Hand::Empty::<Value>,
                    right: Hand::Empty,
                    result: AssertionResult::Unprocessable(UnprocessableReason::MissingJsonBody),
                };
                assertion.assert(&self.log_settings);

                return self;
            };

            // Check for unprocessable json path
            let jsonpath_value = match json_body.path(path) {
                Ok(json) => json,
                Err(_) => {
                    let assertion = Assertion {
                        part: Part::JsonPath,
                        predicate: expr.predicate,
                        left: Hand::Empty::<Value>,
                        right: Hand::Empty,
                        result: AssertionResult::Unprocessable(
                            UnprocessableReason::InvalidJsonPath(path.to_string()),
                        ),
                    };
                    assertion.assert(&self.log_settings);

                    return self;
                }
            };

            let jsonpath_res = JsonPathResult::new(path, jsonpath_value);

            let _assertion = expr
                .value
                .eval(jsonpath_res, expr.predicate, &self.log_settings);
        }

        self
    }

    /// Asserts the response time (in milliseconds).
    pub fn response_time<T>(self, expr: Expression<T>) -> Assert
    where
        T: TimeDsl<u64>,
    {
        if let Some(response_time_ms) = self.response_time_ms {
            let _assertion = expr
                .value
                .eval(response_time_ms, expr.predicate, &self.log_settings);
        }

        self
    }

    /// Asserts the headers of the response.
    pub fn headers<T>(self, expr: Expression<T>) -> Assert
    where
        T: HeadersDsl<HeaderMap>,
    {
        if let Some(headers) = &self.headers {
            let _assertion = expr
                .value
                .eval(headers.clone(), expr.predicate, &self.log_settings);
        }

        self
    }

    /// Asserts a specific header of the response.
    pub fn header<H, T>(self, header_name: H, expr: Expression<T>) -> Assert
    where
        H: AsHeaderName,
        T: HeaderDsl<HeaderValue>,
    {
        if let Some(headers) = self.headers.clone() {
            if let Some(actual_header_val) = headers.get(header_name) {
                let _assertion = expr.value.eval(
                    actual_header_val.clone(),
                    expr.predicate,
                    &self.log_settings,
                );
            } else {
                // Handle missing header name
                let assertion = Assertion {
                    part: Part::Header,
                    predicate: expr.predicate,
                    left: Hand::Empty::<&str>,
                    right: Hand::Empty,
                    result: AssertionResult::Unprocessable(UnprocessableReason::MissingHeader),
                };
                assertion.assert(&self.log_settings);
            }
        }

        self
    }
}
