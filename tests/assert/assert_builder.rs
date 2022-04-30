use std::time::SystemTime;

use crate::HttpMockServer;
use grillon::Result;

#[tokio::test]
async fn custom_response_struct() -> Result<()> {
    use async_trait::async_trait;
    use grillon::{dsl::is_between, header::HeaderMap, Assert, Response, StatusCode};
    use serde_json::Value;

    struct ResponseWrapper {
        pub response: reqwest::Response,
    }

    #[async_trait(?Send)]
    impl Response for ResponseWrapper {
        fn status(&self) -> StatusCode {
            self.response.status()
        }

        async fn json(self) -> Option<Value> {
            self.response.json::<Value>().await.ok()
        }

        fn headers(&self) -> HeaderMap {
            self.response.headers().clone()
        }
    }

    let mock_server = HttpMockServer::new();
    let mock = mock_server.get_valid_user();

    // HTTP call with a different client (grillon uses hyper by default)
    let now = SystemTime::now();
    let response = reqwest::get(mock_server.server.url("/users/1"))
        .await
        .expect("Valid reqwest::Response");
    let response_time = now
        .elapsed()
        .expect("Failed to retrieve response time")
        .as_millis();

    let response_wrapper = ResponseWrapper { response };

    Assert::new(response_wrapper, response_time)
        .await
        .status(is_between(200, 299))
        .assert_fn(|assert| {
            assert!(assert.status == StatusCode::OK, "Bad status code");
        });

    mock.assert();

    Ok(())
}
