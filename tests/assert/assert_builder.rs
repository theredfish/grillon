use crate::HttpMockServer;
use grillon::Result;

#[tokio::test]
async fn custom_response_struct() -> Result<()> {
    use futures::FutureExt;
    use grillon::{header::HeaderMap, Assert, Response, StatusCode};
    use serde_json::Value;
    use std::{future::Future, pin::Pin};

    struct ResponseWrapper {
        pub response: reqwest::Response,
    }

    impl Response for ResponseWrapper {
        fn status(&self) -> StatusCode {
            self.response.status()
        }

        fn json(self) -> Pin<Box<dyn Future<Output = Option<Value>>>> {
            async { self.response.json::<Value>().await.ok() }.boxed_local()
        }

        fn headers(&self) -> HeaderMap {
            self.response.headers().clone()
        }
    }

    let mock_server = HttpMockServer::new();
    let mock = mock_server.get_valid_user();

    // HTTP call with a different client (grillon uses hyper by default)
    let response = reqwest::get(mock_server.server.url("/users/1"))
        .await
        .expect("Valid reqwest::Response");
    let response_wrapper = ResponseWrapper { response };

    Assert::new(response_wrapper).await.status_success();
    mock.assert();

    Ok(())
}
