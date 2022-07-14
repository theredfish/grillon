//! The `assert` module provides everything to assert parts of http responses with built-in matchers.
//! The `diff` feature can be activated to display pretty assertions.
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
//!     let response = reqwest::get("http://jsonplaceholder.typicode.com/users/1")
//!         .await
//!         .expect("Valid reqwest::Response");
//!     let response_wrapper = ResponseWrapper { response };
//!
//!     Assert::new(response_wrapper).await.status_success();
//!
//!     Ok(())
//!  }
//! ```

pub mod body;
pub mod header;

use self::header::{HeadersAbsentMatcher, HeadersExistMatcher};
use crate::dsl::{http::*, Expression};
use crate::Response;
use http::HeaderMap;
use hyper::StatusCode;
#[cfg(feature = "diff")]
use pretty_assertions::assert_eq;
use serde_json::Value;

pub struct Assertion {
    pub result: bool,
    pub message: String,
}

impl Assertion {
    pub fn assert(self) {
        assert!(self.result, "{}", self.message);
    }
}

/// The `Assert` uses an internal representation of the
/// http response to assert it.
pub struct Assert {
    pub headers: HeaderMap,
    pub status: StatusCode,
    pub json: Option<Value>,
    pub response_time_ms: u128,
}

impl Assert {
    /// Creates an `Assert` instance with an internal representation
    /// of the given response to assert.
    pub async fn new<T>(response: T, response_time_ms: u128) -> Self
    where
        T: Response,
    {
        Assert {
            headers: response.headers().clone(),
            status: response.status(),
            json: response.json().await,
            response_time_ms,
        }
    }

    /// Extends the built-in assertions with a custom assertion.
    /// The closure gives access to the [`Assert`] instance.
    ///
    /// # Example
    ///
    /// ```rust
    /// # use grillon::{Grillon, Result, StatusCode, dsl::is};
    /// # async fn custom_assert() -> Result<()> {
    /// Grillon::new("http://jsonplaceholder.typicode.com")?
    ///     .get("/users")
    ///     .assert()
    ///     .await
    ///     .status_success()
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

    /// Asserts that the response status is equals to the given one.
    pub fn status<T>(self, expr: Expression<T>) -> Assert
    where
        T: StatusCodeDsl<StatusCode>,
    {
        expr.value.eval(&self.status, expr.operator).assert();

        self
    }

    /// Asserts that the response status is successful (200-299).
    pub fn status_success(self) -> Assert {
        assert!(
            self.status.is_success(),
            "200-299 status expected, found {}",
            self.status.as_u16()
        );
        self
    }

    /// Asserts that the response status is a client error (400-499).
    pub fn status_client_error(self) -> Assert {
        assert!(
            self.status.is_client_error(),
            "400-499 status expected, found {}",
            self.status.as_u16()
        );
        self
    }

    /// Asserts that the response status is a server error (500-599).
    pub fn status_server_error(self) -> Assert {
        assert!(
            self.status.is_server_error(),
            "500-599 status expected, found {}",
            self.status.as_u16()
        );
        self
    }

    pub fn json_body<T>(self, expr: Expression<T>) -> Assert
    where
        T: JsonBodyDsl<Value>,
    {
        let actual = self.json.as_ref().unwrap();
        expr.value.eval(actual, expr.operator).assert();

        self
    }

    pub fn response_time<T>(self, expr: Expression<T>) -> Assert
    where
        T: TimeDsl<u128>,
    {
        expr.value
            .eval(&self.response_time_ms, expr.operator)
            .assert();

        self
    }

    pub fn headers<T>(self, expr: Expression<T>) -> Assert
    where
        T: HeaderDsl<HeaderMap>,
    {
        expr.value.eval(&self.headers, expr.operator).assert();

        self
    }

    /// Asserts that the headers exist in the response headers.
    pub fn headers_exist<H: HeadersExistMatcher + std::fmt::Debug>(self, headers: H) -> Assert {
        headers.exist(&self.headers);

        self
    }

    /// Asserts that the headers are absent from the response headers.
    pub fn headers_absent<H: HeadersAbsentMatcher + std::fmt::Debug>(self, headers: H) -> Assert {
        headers.absent(&self.headers);

        self
    }
}
