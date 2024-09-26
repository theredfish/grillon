use std::{str::FromStr, time::Instant};

use crate::HttpMockServer;
use grillon::{LogSettings, Result};
use http::{HeaderName, HeaderValue};

#[tokio::test]
async fn custom_response_struct() -> Result<()> {
    use async_trait::async_trait;
    use grillon::{dsl::is_between, header::HeaderMap, Assert, Response, StatusCode};
    use serde_json::Value;

    struct ResponseWrapper {
        pub response: surf::Response,
    }

    #[async_trait(?Send)]
    impl Response for ResponseWrapper {
        fn status(&self) -> StatusCode {
            let status: u16 = self.response.status().into();
            StatusCode::from_u16(status).expect("Invalid status code range")
        }

        async fn json(mut self) -> Option<Value> {
            if let Ok(bytes) = &self.response.body_bytes().await {
                return serde_json::from_slice::<Value>(bytes).ok();
            }

            None
        }

        fn headers(&self) -> HeaderMap {
            let mut headers = HeaderMap::new();

            let keys = self
                .response
                .header_names()
                .map(|k| {
                    HeaderName::from_str(k.as_str()).expect("Failed to convert Surf header name")
                })
                .collect::<Vec<HeaderName>>();

            let values = self
                .response
                .header_values()
                .map(|v| {
                    HeaderValue::from_str(v.as_str()).expect("Failed to convert Surf header value")
                })
                .collect::<Vec<HeaderValue>>();

            assert_eq!(
                keys.len(),
                values.len(),
                "surf header names vector lenght doesn't match the values length"
            );

            for (k, v) in keys.iter().zip(values.iter()) {
                headers.insert(k.clone(), v.clone());
            }

            headers
        }
    }

    let mock_server = HttpMockServer::new();
    let mock = mock_server.get_valid_user();

    // HTTP call with a different client (grillon uses reqwest by default)
    let now = Instant::now();
    let response = surf::get(mock_server.server.url("/users/1"))
        .await
        .expect("Valid surf::Response");
    let response_time_ms = now.elapsed().as_millis() as u64;

    let response_wrapper = ResponseWrapper { response };

    Assert::new(
        Some(response_wrapper),
        Some(response_time_ms),
        LogSettings::default(),
    )
    .await
    .status(is_between(200, 299))
    .assert_fn(|assert| {
        assert!(assert.status == Some(StatusCode::OK), "Bad status code");
    });

    mock.assert();

    Ok(())
}
