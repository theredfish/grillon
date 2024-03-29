use crate::HttpMockServer;
use grillon::{dsl::http::is_success, Grillon, Result, StatusCode};

#[tokio::test]
async fn custom_assert() -> Result<()> {
    let mock_server = HttpMockServer::new();
    let mock = mock_server.get_valid_user();

    Grillon::new(mock_server.server.url("/").as_ref())?
        .get("users/1")
        .assert()
        .await
        .assert_fn(|assert| {
            assert!(!assert.headers.is_empty());
            assert!(assert.status == StatusCode::OK);
            assert!(assert.json.is_some());

            println!("Json response : {:#?}", assert.json);
        })
        .status(is_success());

    mock.assert();

    Ok(())
}
