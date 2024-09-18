use crate::HttpMockServer;
use grillon::dsl::{
    contains,
    http::{is_client_error, is_success},
};
use grillon::{
    header::{HeaderValue, SET_COOKIE},
    Grillon, Result,
};

#[tokio::test]
async fn cookies_should_be_stored() -> Result<()> {
    let mock_server = HttpMockServer::new();
    let auth_mock = mock_server.auth();
    let auth_endpoint_mock = mock_server.authenticated_request();

    let grillon = Grillon::new(&mock_server.server.url("/"))?.store_cookies(true)?;

    grillon.post("auth").assert().await.headers(contains(vec![(
        SET_COOKIE,
        HeaderValue::from_static("SESSIONID=123; HttpOnly"),
    )]));

    grillon
        .get("authenticated/endpoint")
        .assert()
        .await
        .status(is_success());

    auth_mock.assert();
    auth_endpoint_mock.assert();

    Ok(())
}

#[tokio::test]
#[should_panic]
async fn disabled_cookie_store_should_not_send_cookies() {
    let mock_server = HttpMockServer::new();
    let auth_mock = mock_server.auth();
    let auth_endpoint_mock = mock_server.authenticated_request();

    let grillon = Grillon::new(&mock_server.server.url("/")).unwrap();

    grillon.post("auth").assert().await.headers(contains(vec![(
        SET_COOKIE,
        HeaderValue::from_static("SESSIONID=123; HttpOnly"),
    )]));

    grillon
        .store_cookies(false)
        .unwrap()
        .get("authenticated/endpoint")
        .assert()
        .await
        .status(is_client_error());

    auth_mock.assert();
    // This should panic because the SESSIONID cookie isn't set.
    auth_endpoint_mock.assert();
}

#[tokio::test]
#[should_panic]
async fn missing_cookies_should_panic() {
    let mock_server = HttpMockServer::new();
    let auth_mock = mock_server.auth();
    let auth_endpoint_mock = mock_server.authenticated_request();

    let grillon = Grillon::new(&mock_server.server.url("/")).unwrap();

    grillon.post("auth").assert().await.headers(contains(vec![(
        SET_COOKIE,
        HeaderValue::from_static("SESSIONID=123; HttpOnly"),
    )]));

    grillon
        .get("authenticated/endpoint")
        .assert()
        .await
        .status(is_client_error());

    auth_mock.assert();
    // This should panic because the SESSIONID cookie isn't set.
    auth_endpoint_mock.assert();
}
