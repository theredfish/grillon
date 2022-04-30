use crate::HttpMockServer;
use grillon::{
    dsl::{is, is_not},
    Grillon, Result, StatusCode,
};

#[tokio::test]
async fn status_success() -> Result<()> {
    let mock_server = HttpMockServer::new();
    let mock = mock_server.delete_valid_user();

    Grillon::new(mock_server.server.url("/").as_ref())?
        .delete("users/1")
        .assert()
        .await
        .status_success()
        .status(is(StatusCode::NO_CONTENT));

    mock.assert();

    Ok(())
}

#[tokio::test]
async fn status_client_error() -> Result<()> {
    let mock_server = HttpMockServer::new();

    Grillon::new(mock_server.server.url("/").as_ref())?
        .get("inexistant/resource")
        .assert()
        .await
        .status_client_error()
        .status(is(StatusCode::NOT_FOUND));

    Ok(())
}

#[tokio::test]
async fn status_server_error() -> Result<()> {
    let mock_server = HttpMockServer::new();
    mock_server.server_error();

    Grillon::new(mock_server.server.url("/").as_ref())?
        .get("server/error")
        .assert()
        .await
        .status_server_error()
        .status(is(StatusCode::INTERNAL_SERVER_ERROR));

    Ok(())
}

#[tokio::test]
#[should_panic]
async fn unexpected_status() {
    let mock_server = HttpMockServer::new();

    Grillon::new(mock_server.server.url("/").as_ref())
        .unwrap()
        .get("some/path")
        .assert()
        .await
        .status(is(StatusCode::OK));
}

#[tokio::test]
async fn status_is_not() -> Result<()> {
    let mock_server = HttpMockServer::new();
    let mock = mock_server.delete_valid_user();

    Grillon::new(mock_server.server.url("/").as_ref())?
        .delete("users/1")
        .assert()
        .await
        .status_success()
        .status(is_not(500))
        .status(is_not(StatusCode::INTERNAL_SERVER_ERROR));

    mock.assert();

    Ok(())
}
