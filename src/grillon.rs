use crate::assert::Assert;
use crate::error::Result;
use hyper::{
    body::{Body, Buf},
    client::HttpConnector,
    header::{HeaderMap, HeaderName, HeaderValue},
    http::{request::Request as HyperRequest, response::Response as HyperResponse, StatusCode},
    Client, Method, Uri,
};
use serde_json::Value;
use std::{future::Future, pin::Pin};

pub struct Grillon {
    pub base_url: Uri,
    pub client: Client<HttpConnector>,
}

pub struct Request<'c> {
    pub method: Method,
    pub uri: Uri,
    pub headers: HeaderMap,
    pub payload: Option<Body>,
    pub client: &'c Client<HttpConnector>,
}

const METHODS_NO_BODY: &[Method] = &[
    Method::CONNECT,
    Method::HEAD,
    Method::GET,
    Method::OPTIONS,
    Method::TRACE,
];

pub trait RequestHeaders {
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

pub trait Response {
    fn status(&self) -> StatusCode;
    fn json(self) -> Pin<Box<dyn Future<Output = Option<Value>> + Send>>;
    fn headers(&self) -> HeaderMap;
}

impl Response for HyperResponse<Body> {
    fn status(&self) -> StatusCode {
        self.status()
    }

    fn json(self) -> Pin<Box<dyn Future<Output = Option<Value>> + Send>> {
        let (_, body) = self.into_parts();
        let body = hyper::body::aggregate(body);

        let json = async move {
            let body = body.await.expect("Valid buffer");
            if !body.has_remaining() {
                return None;
            }

            let json: Value =
                serde_json::from_reader(body.reader()).expect("Failed to decode json");

            Some(json)
        };

        Box::pin(json)
    }

    fn headers(&self) -> HeaderMap {
        self.headers().clone()
    }
}

impl Grillon {
    pub fn new(api_base_url: &str) -> Result<Grillon> {
        Ok(Grillon {
            base_url: api_base_url.parse::<Uri>()?,
            client: Client::builder().build_http(),
        })
    }

    pub fn get(&self, path: &str) -> Request {
        self.request(Method::GET, path)
    }

    pub fn post(&self, path: &str) -> Request {
        self.request(Method::POST, path)
    }

    pub fn put(&self, path: &str) -> Request {
        self.request(Method::PUT, path)
    }

    pub fn patch(&self, path: &str) -> Request {
        self.request(Method::PATCH, path)
    }

    pub fn delete(&self, path: &str) -> Request {
        self.request(Method::DELETE, path)
    }

    pub fn request(&self, method: Method, path: &str) -> Request {
        let uri = crate::url::concat(&self.base_url, path).unwrap_or_else(|err| panic!("{}", err));

        Request {
            method,
            uri,
            headers: HeaderMap::new(),
            payload: None,
            client: &self.client,
        }
    }
}

impl Request<'_> {
    pub async fn assert(self) -> Assert {
        let mut req = HyperRequest::new(self.payload.unwrap_or_else(Body::empty));
        *req.method_mut() = self.method;
        *req.headers_mut() = self.headers;
        *req.uri_mut() = self.uri;

        let response = self.client.request(req).await.expect("valid response");

        Assert::new(response).await
    }

    pub fn headers<H: RequestHeaders>(mut self, headers: H) -> Self {
        self.headers = headers.to_header_map();

        self
    }

    pub fn payload(mut self, json: Value) -> Self {
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
}

#[cfg(test)]
mod tests {}
