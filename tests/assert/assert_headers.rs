use crate::HttpMockServer;
use grillon::{
    header::{HeaderMap, HeaderValue, CONTENT_TYPE},
    Grillon, Result,
};

#[tokio::test]
async fn headers_exist() -> Result<()> {
    let mock_server = HttpMockServer::new();
    let mock = mock_server.get_valid_user();
    let vec_header_map = vec![(CONTENT_TYPE, HeaderValue::from_static("application/json"))];
    let mut header_map = HeaderMap::new();
    header_map.insert(CONTENT_TYPE, HeaderValue::from_static("application/json"));

    Grillon::new(mock_server.server.url("/").as_ref())?
        .get("users/1")
        .assert()
        .await
        .headers_exist(vec_header_map)
        .headers_exist(header_map);

    mock.assert();

    Ok(())
}

#[tokio::test]
async fn headers_absent() -> Result<()> {
    let mock_server = HttpMockServer::new();
    let mock = mock_server.get_valid_user();
    let vec_header_map = vec![(CONTENT_TYPE, HeaderValue::from_static("text/html"))];
    let mut header_map = HeaderMap::new();
    header_map.insert(CONTENT_TYPE, HeaderValue::from_static("text/html"));

    Grillon::new(mock_server.server.url("/").as_ref())?
        .get("users/1")
        .assert()
        .await
        .headers_absent(vec_header_map)
        .headers_absent(header_map);

    mock.assert();

    Ok(())
}

#[tokio::test]
async fn headers_check_empty_against_not_empty() -> Result<()> {
    let mock_server = HttpMockServer::new();
    let mock = mock_server.get_empty_response();

    // The MockServer always returns the content type and the date in headers
    Grillon::new(mock_server.server.url("/").as_ref())?
        .get("empty")
        .assert()
        .await
        .headers_exist(vec![]);

    mock.assert();

    Ok(())
}
