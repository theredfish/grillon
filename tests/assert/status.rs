use crate::HttpMockServer;
use grillon::{
    dsl::{
        http::{is_client_error, is_server_error, is_success},
        is, is_between, is_not,
    },
    Grillon, LogSettings, Result, StatusCode,
};

#[tokio::test]
async fn status_success() -> Result<()> {
    let mock_server = HttpMockServer::new();
    let mock = mock_server.delete_valid_user();

    Grillon::new(&mock_server.server.url("/"))?
        .delete("users/1")
        .assert()
        .await
        .status(is_between(200, 299))
        .status(is_success())
        .status(is(StatusCode::NO_CONTENT));

    mock.assert();

    Ok(())
}

#[tokio::test]
async fn status_client_error() -> Result<()> {
    let mock_server = HttpMockServer::new();

    Grillon::new(&mock_server.server.url("/"))?
        .get("inexistant/resource")
        .assert()
        .await
        .status(is_between(400, 499))
        .status(is_client_error())
        .status(is(StatusCode::NOT_FOUND));

    Ok(())
}

#[tokio::test]
async fn status_server_error() -> Result<()> {
    let mock_server = HttpMockServer::new();
    mock_server.server_error();

    Grillon::new(&mock_server.server.url("/"))?
        .get("server/error")
        .assert()
        .await
        .status(is_between(500, 599))
        .status(is_server_error())
        .status(is(StatusCode::INTERNAL_SERVER_ERROR));

    Ok(())
}

#[tokio::test]
#[should_panic]
async fn unexpected_status() {
    let mock_server = HttpMockServer::new();

    Grillon::new(&mock_server.server.url("/"))
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

    Grillon::new(&mock_server.server.url("/"))?
        .log_settings(LogSettings::StdAssert)
        .delete("users/1")
        .assert()
        .await
        .status(is_between(200, 299))
        .status(is_success())
        .status(is_not(500))
        .status(is_not(StatusCode::INTERNAL_SERVER_ERROR));

    mock.assert();

    Ok(())
}

#[tokio::test]
async fn status_is_between() -> Result<()> {
    let mock_server = HttpMockServer::new();
    let mock = mock_server.delete_valid_user();

    Grillon::new(&mock_server.server.url("/"))?
        .delete("users/1")
        .assert()
        .await
        .status(is_success())
        .status(is_between(200, 204))
        .status(is_between(StatusCode::OK, StatusCode::NO_CONTENT));

    mock.assert();

    Ok(())
}
