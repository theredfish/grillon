use self::core::body::ExpectBody;
pub use self::core::errors::{Error, Result};
use hyper::body::{Body, Buf};
use hyper::{http::request::Builder as RequestBuilder, http::response::Parts};
use hyper::{Client, Method, Uri};

mod core;

pub use hyper::StatusCode;
pub use serde_json::{json, Value};

pub struct HttpRequest {
    pub builder: RequestBuilder,
    pub payload: Option<Body>,
}

#[derive(Debug)]
pub struct HttpResponse {
    parts: Parts,
    data: Value,
}

pub struct ApiHours {
    base_url: String,
    request: Result<HttpRequest>,
}

pub struct ApiHoursExpect {
    response: HttpResponse,
}

impl ApiHours {
    pub fn new(base_url: &str) -> ApiHours {
        ApiHours {
            base_url: base_url.to_string(),
            request: Err(Error::IncompleteHttRequest),
        }
    }

    pub fn get(mut self, path: &str) -> ApiHours {
        let uri: Uri = format!("{}{}", self.base_url, path)
            .parse()
            .expect("Bad url syntax");

        let builder = RequestBuilder::new().uri(uri).method(Method::GET);
        self.request = Ok(HttpRequest {
            builder,
            payload: None,
        });

        self
    }

    pub fn post(mut self, path: &str) -> ApiHours {
        let uri: Uri = format!("{}{}", self.base_url, path)
            .parse()
            .expect("Bad url syntax");

        let builder = RequestBuilder::new().uri(uri).method(Method::POST);

        self.request = Ok(HttpRequest {
            builder,
            payload: None,
        });

        self
    }

    pub fn payload(mut self, json: Value) -> ApiHours {
        self.request
            .as_mut()
            .expect("Cannot build the request.")
            .payload = Some(Body::from(json.to_string()));
        self
    }

    pub async fn verify(self) -> Result<ApiHoursExpect> {
        let http_request = self.request?;
        let builder = http_request.builder;

        let request = match http_request.payload {
            Some(payload) => builder.body(payload),
            None => builder.body(Body::empty()),
        };

        let request = request.expect("Cannot build request");

        // TODO : Reuse a lazy static client
        let response = Client::new()
            .request(request)
            .await
            .expect("Cannot send request");

        // Aggregate the body...
        let (parts, body) = response.into_parts();
        let whole_body = hyper::body::aggregate(body).await.expect("nope");
        // Decode as JSON...
        let data: serde_json::Value =
            serde_json::from_reader(whole_body.reader()).expect("Can't decode json");

        let http_response = HttpResponse { parts, data };
        println!("Verify response : {:#?}", http_response);

        Ok(ApiHoursExpect {
            response: http_response,
        })
    }
}

impl ApiHoursExpect {
    pub fn status(self, expected: StatusCode) -> ApiHoursExpect {
        let actual = self.response.parts.status;
        assert_eq!(
            actual, expected,
            "Actual status {} is not equals to expected status {}",
            actual, expected
        );
        self
    }

    pub fn status_success(self) -> ApiHoursExpect {
        assert!(self.response.parts.status.is_success());
        self
    }

    pub fn status_client_error(self) -> ApiHoursExpect {
        assert!(self.response.parts.status.is_client_error());
        self
    }

    pub fn status_server_error(self) -> ApiHoursExpect {
        assert!(self.response.parts.status.is_server_error());
        self
    }

    pub fn body<T: ExpectBody>(self, expected: T) -> ApiHoursExpect {
        let actual = self.response.data.clone();
        let expected = expected.to_value();

        assert_eq!(actual, expected);

        self
    }
}

#[cfg(test)]
mod tests {}
