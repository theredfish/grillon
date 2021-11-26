use crate::HttpMockServer;
use grillon::{header::HeaderMap, Assert, Error, Response, StatusCode};

#[tokio::test]
async fn custom_response_struct() -> Result<(), Error> {
    use serde_json::Value;
    use std::{future::Future, pin::Pin};

    struct MyResponse {
        status: u16,
        body: serde_json::Value,
        headers: HeaderMap,
    }

    let mock_server = HttpMockServer::new();
    let mock = mock_server.get_valid_user();

    // HTTP call with a different client (grillon uses hyper by default)
    let response = reqwest::get(mock_server.server.url("/users/1"))
        .await
        .expect("Valid reqwest::Response");

    // Build a custom Response with reqwest response
    let status = response.status().as_u16();
    let headers = response.headers().clone();
    let body: Value = response.json().await.expect("Valid json");

    let response = MyResponse {
        status,
        headers,
        body,
    };

    impl Response for MyResponse {
        fn status(&self) -> StatusCode {
            StatusCode::from_u16(self.status).expect("Valid status code from u16")
        }

        fn json(self) -> Pin<Box<dyn Future<Output = Option<Value>> + Send>> {
            let json = async move { Some(self.body) };

            Box::pin(json)
        }

        fn headers(&self) -> HeaderMap {
            self.headers.clone()
        }
    }

    Assert::new(response).await.status_success();
    mock.assert();

    Ok(())
}
