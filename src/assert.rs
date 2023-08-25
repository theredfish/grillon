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

use crate::{
    assertion::{Assertion, AssertionResult, Hand, UnprocessableReason},
    dsl::{
        http::*,
        json_path::{JsonPathDsl, JsonPathResult},
        Expression, Part,
    },
    grillon::LogSettings,
    Response,
};
use http::HeaderMap;
use hyper::StatusCode;
use serde_json::Value;

/// [`Assert`] uses an internal representation of the http response to assert
/// against.
#[derive(Clone)]
pub struct Assert {
    /// The http response header to assert.
    pub headers: HeaderMap,
    /// The http response status to assert.
    pub status: StatusCode,
    /// The http response json body to assert.
    pub json: Option<Value>,
    /// The http response time (in milliseconds) to assert.
    pub response_time_ms: u64,
    /// The test results output.
    pub log_settings: LogSettings,
}

impl Assert {
    /// Creates an `Assert` instance with an internal representation
    /// of the given response to assert.
    pub async fn new<T>(response: T, response_time_ms: u64, log_settings: LogSettings) -> Self
    where
        T: Response,
    {
        Assert {
            headers: response.headers().clone(),
            status: response.status(),
            json: response.json().await,
            response_time_ms,
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
        let _assertion = expr
            .value
            .eval(self.status, expr.predicate, &self.log_settings);

        self
    }

    /// Asserts the json body of the response.
    pub fn json_body<T>(self, expr: Expression<T>) -> Assert
    where
        T: JsonBodyDsl<Value>,
    {
        let actual = if let Some(body) = self.json.clone() {
            body
        } else {
            let assertion = Assertion {
                part: Part::JsonPath,
                predicate: expr.predicate,
                left: Hand::Empty::<Value>,
                right: Hand::Empty,
                result: AssertionResult::Unprocessable(UnprocessableReason::JsonBodyMissing),
            };
            assertion.assert(&self.log_settings);

            return self;
        };
        let _assertion = expr.value.eval(actual, expr.predicate, &self.log_settings);

        self
    }

    /// Asserts the value found at the given json path.
    pub fn json_path<T>(self, path: &str, expr: Expression<T>) -> Assert
    where
        T: JsonPathDsl<Value>,
    {
        use jsonpath_rust::JsonPathQuery;

        // Check for empty json body
        let json_body = if let Some(body) = self.json.clone() {
            body
        } else {
            let assertion = Assertion {
                part: Part::JsonPath,
                predicate: expr.predicate,
                left: Hand::Empty::<Value>,
                right: Hand::Empty,
                result: AssertionResult::Unprocessable(UnprocessableReason::JsonBodyMissing),
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
                    result: AssertionResult::Unprocessable(UnprocessableReason::InvalidJsonPath(
                        path.to_string(),
                    )),
                };
                assertion.assert(&self.log_settings);

                return self;
            }
        };

        let jsonpath_res = JsonPathResult::new(path, jsonpath_value);

        let _assertion = expr
            .value
            .eval(jsonpath_res, expr.predicate, &self.log_settings);

        self
    }

    /// Asserts the response time (in milliseconds).
    pub fn response_time<T>(self, expr: Expression<T>) -> Assert
    where
        T: TimeDsl<u64>,
    {
        let _assertion = expr
            .value
            .eval(self.response_time_ms, expr.predicate, &self.log_settings);

        self
    }

    /// Asserts the headers of the response.
    pub fn headers<T>(self, expr: Expression<T>) -> Assert
    where
        T: HeadersDsl<HeaderMap>,
    {
        let _assertion = expr
            .value
            .eval(self.headers.clone(), expr.predicate, &self.log_settings);

        self
    }
}
