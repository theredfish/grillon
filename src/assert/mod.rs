mod body;
mod header;

use self::{
    body::BodyMatch,
    header::{HeadersExcept, HeadersMatch},
};
use crate::mantis::Response;
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
        assert_eq!(self.status, expected, "{} is expected", expected);
        self
    }

    pub fn status_success(self) -> Assert {
        assert!(
            self.status.is_success(),
            "Status {} expected, found {}",
            StatusCode::OK,
            self.status
        );
        self
    }

    pub fn status_client_error(self) -> Assert {
        assert!(self.status.is_client_error());
        self
    }

    pub fn status_server_error(self) -> Assert {
        assert!(self.status.is_server_error());
        self
    }

    pub fn body(self, body: impl BodyMatch) -> Assert {
        if let Some(json) = self.json.as_ref() {
            assert!(body.matches(json));
        } else {
            panic!("There is no json body to compare against.");
        }

        self
    }

    pub fn headers_eq<H: HeadersMatch>(self, headers: H) -> Assert {
        assert!(headers.matches(&self.headers));

        self
    }

    pub fn headers_ne<H: HeadersExcept>(self, headers: H) -> Assert {
        assert!(headers.except(&self.headers));

        self
    }
}
