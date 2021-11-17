use crate::assert::Assert;
use crate::error::Result;
use http::StatusCode;
use hyper::body::{Body, Buf};
use hyper::header::{self, HeaderMap, HeaderValue};
use hyper::{client::HttpConnector, Client, Method, Uri};
use hyper::{http::request::Builder, http::response::Response as HyperResponse};
use serde_json::Value;
use std::{future::Future, pin::Pin};

pub struct Mantis {
    pub base_url: Uri,
    pub client: Client<HttpConnector>,
}

pub struct Request<'c> {
    pub method: Method,
    pub uri: Uri,
    pub payload: Option<Body>,
    pub client: &'c Client<HttpConnector>,
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
            let json: Value = serde_json::from_reader(body.reader()).expect("Can't decode json");

            Some(json)
        };

        Box::pin(json)
    }

    fn headers(&self) -> HeaderMap {
        self.headers().clone()
    }
}

impl Mantis {
    pub fn new(url: &str) -> Result<Mantis> {
        let mut headers: HeaderMap<HeaderValue> = HeaderMap::with_capacity(2);
        headers.insert(header::ACCEPT, HeaderValue::from_static("*/*"));

        let base_url = url.parse::<Uri>()?;

        Ok(Mantis {
            base_url,
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

    pub fn delete(&self, path: &str) -> Request {
        self.request(Method::DELETE, path)
    }

    pub fn request(&self, method: Method, path: &str) -> Request {
        let uri = crate::url::concat(&self.base_url, path).unwrap_or_else(|err| panic!(err));

        Request {
            method,
            uri,
            payload: None,
            client: &self.client,
        }
    }
}

impl Request<'_> {
    pub async fn assert(self) -> Assert {
        let builder = Builder::new().method(self.method).uri(self.uri);

        let req = builder
            .body(self.payload.unwrap_or_else(|| Body::empty()))
            .expect("valid body");
        let response = self.client.request(req).await.expect("valid response");

        Assert::new(response).await
    }

    pub fn payload(mut self, json: Value) -> Self {
        match self.method {
            Method::GET => {
                println!(
                    "{} does not support HTTP body. No payload will be sent.",
                    self.method
                );
                return self;
            }
            _ => (),
        }

        self.payload = Some(Body::from(json.to_string()));

        self
    }
}

#[cfg(test)]
mod tests {}
