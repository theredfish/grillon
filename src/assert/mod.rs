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

use self::{
    body::BodyExactMatcher,
    header::{HeadersAbsentMatcher, HeadersExistMatcher},
};
use crate::Response;
use http::HeaderMap;
use hyper::StatusCode;
#[cfg(feature = "diff")]
use pretty_assertions::assert_eq;
use serde_json::Value;

/// The `Assert` uses an internal representation of the
/// http response to assert it.
pub struct Assert {
    pub headers: HeaderMap,
    pub status: StatusCode,
    pub json: Option<Value>,
}

impl Assert {
    /// Creates an `Assert` instance with an internal representation
    /// of the given response to assert.
    pub async fn new<T>(response: T) -> Self
    where
        T: Response,
    {
        let headers = response.headers().clone();
        let status = response.status();
        let json = response.json().await;

        Assert {
            headers,
            status,
            json,
        }
    }

    /// Extends the built-in assertions with a custom assertion.
    /// The closure gives access to the [`Assert`] instance.
    ///
    /// # Example
    ///
    /// ```rust
    /// # use grillon::{Grillon, Result, StatusCode};
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
    ///      .status(StatusCode::CREATED);
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
    pub fn status(self, expected: StatusCode) -> Assert {
        assert_eq!(
            expected,
            self.status,
            "{} status expected, found {}",
            expected.as_u16(),
            self.status.as_u16()
        );
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

    /// Asserts that the response body matches exactly the given one.
    pub fn body<B: BodyExactMatcher + std::fmt::Debug>(self, body: B) -> Assert {
        body.matches(self.json.as_ref());

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
