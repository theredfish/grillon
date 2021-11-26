mod body;
mod header;

use self::{
    body::BodyMatch,
    header::{HeadersExcept, HeadersExist},
};
use crate::grillon::Response;
use http::HeaderMap;
use hyper::StatusCode;
use serde_json::Value;

pub struct Assert {
    pub headers: HeaderMap,
    pub status: StatusCode,
    pub json: Option<Value>,
}

impl Assert {
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

    pub fn status(self, expected: StatusCode) -> Assert {
        assert_eq!(
            self.status, expected,
            "{} status expected, found {}",
            expected, self.status
        );
        self
    }

    pub fn status_success(self) -> Assert {
        assert!(
            self.status.is_success(),
            "200-299 status expected, found {}",
            self.status
        );
        self
    }

    pub fn status_client_error(self) -> Assert {
        assert!(
            self.status.is_client_error(),
            "400-499 status expected, found {}",
            self.status
        );
        self
    }

    pub fn status_server_error(self) -> Assert {
        assert!(
            self.status.is_server_error(),
            "500-599 status expected, found {}",
            self.status
        );
        self
    }

    pub fn body<B: BodyMatch + std::fmt::Debug>(self, body: B) -> Assert {
        let json = self.json.as_ref();
        assert!(
            body.matches(json),
            "The json body doesn't match the expected one. Expected : {:#?}, Found = {:#?}",
            body,
            json,
        );

        self
    }

    pub fn headers_exist<H: HeadersExist + std::fmt::Debug>(self, headers: H) -> Assert {
        assert!(
            headers.exist(&self.headers),
            "One or more headers do not match the expected headers. Expected : {:#?}, Found : {:#?}",
            headers,
            self.headers,
        );

        self
    }

    pub fn headers_except<H: HeadersExcept + std::fmt::Debug>(self, headers: H) -> Assert {
        assert!(
            headers.except(&self.headers),
            "One or more headers match the given headers while they are unexpected. Expected : {:#?}, Found : {:#?}",
            headers,
            self.headers,
        );

        self
    }
}
