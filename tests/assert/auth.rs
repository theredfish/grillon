use crate::HttpMockServer;
use grillon::dsl::http::is_success;
use grillon::{Grillon, Result};

#[tokio::test]
async fn it_should_set_bearer_auth_header() -> Result<()> {
    let mock_server = HttpMockServer::new();
    let bearer_auth = mock_server.bearer_auth();

    Grillon::new(&mock_server.server.url("/"))?
        .get("auth/bearer/endpoint")
        .bearer_auth("token-123")
        .assert()
        .await
        .status(is_success());

    bearer_auth.assert();

    Ok(())
}

#[tokio::test]
async fn it_should_set_basic_auth_header() -> Result<()> {
    let mock_server = HttpMockServer::new();
    let basic_auth = mock_server.basic_auth();

    Grillon::new(&mock_server.server.url("/"))?
        .get("auth/basic/endpoint")
        .basic_auth("isaac", Some("rayne"))
        .assert()
        .await
        .status(is_success());

    basic_auth.assert();

    Ok(())
}
