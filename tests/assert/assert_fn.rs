use crate::HttpMockServer;
use grillon::{dsl::http::is_success, Assert, Grillon, Result, StatusCode};

#[tokio::test]
async fn custom_assert() -> Result<()> {
    let mock_server = HttpMockServer::new();
    let mock = mock_server.get_valid_user();

    Grillon::new(mock_server.server.url("/").as_ref())?
        .get("users/1")
        .assert()
        .await
        .assert_fn(|assert| {
            let Assert {
                headers,
                status,
                json,
                ..
            } = assert.clone();

            if let Some(headers) = headers {
                assert!(!headers.is_empty());
            }

            assert!(status == Some(StatusCode::OK));

            if let Some(json) = json {
                assert!(json.is_some());
                println!("Json response : {:#?}", json);
            }
        })
        .status(is_success());

    mock.assert();

    Ok(())
}
