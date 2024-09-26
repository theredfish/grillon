use crate::HttpMockServer;
use grillon::{
    dsl::{contains, does_not_contain, is, is_not},
    header::{HeaderMap, HeaderValue, CONTENT_LENGTH, CONTENT_TYPE, DATE},
    Grillon, Method, Result,
};

#[tokio::test]
async fn headers_equality() -> Result<()> {
    let mock_server = HttpMockServer::new();
    let mock = mock_server.get_valid_user();
    let mut header_map = HeaderMap::new();
    let mut header_vec = Vec::new();
    let (content_type, content_length, date) = (
        HeaderValue::from_static("application/json"),
        HeaderValue::from_static("23"),
        HeaderValue::from_static("today"),
    );

    header_map.insert(CONTENT_TYPE, content_type.clone());
    header_map.insert(CONTENT_LENGTH, content_length.clone());
    header_map.insert(DATE, date.clone());
    header_vec.push((CONTENT_TYPE, content_type));
    header_vec.push((CONTENT_LENGTH, content_length.clone()));
    header_vec.push((DATE, date));

    Grillon::new(mock_server.server.url("/").as_ref())?
        .get("users/1")
        .assert()
        .await
        .headers(is(header_map))
        .headers(is(header_vec))
        .headers(is(vec![
            ("content-type", "application/json"),
            ("content-length", "23"),
            ("date", "today"),
        ]))
        .headers(is_not(vec![(CONTENT_LENGTH, content_length)]))
        .headers(is_not(vec![("content-length", "23")]));

    mock.assert();

    Ok(())
}

#[tokio::test]
async fn headers_contains() -> Result<()> {
    let mock_server = HttpMockServer::new();
    let mock = mock_server.get_valid_user();
    let vec_header_map = vec![(CONTENT_TYPE, HeaderValue::from_static("application/json"))];
    let mut header_map = HeaderMap::new();
    header_map.insert(CONTENT_TYPE, HeaderValue::from_static("application/json"));

    Grillon::new(mock_server.server.url("/").as_ref())?
        .get("users/1")
        .assert()
        .await
        .headers(contains(vec_header_map))
        .headers(contains(header_map));

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
        .headers(does_not_contain(vec_header_map))
        .headers(does_not_contain(header_map))
        .headers(does_not_contain(vec![("content-type", "text-html")]));

    mock.assert();

    Ok(())
}

#[tokio::test]
async fn headers_check_empty_against_not_empty() -> Result<()> {
    let mock_server = HttpMockServer::new();
    let mock = mock_server.get_empty_response();

    // The MockServer always returns the content type and the date in headers
    Grillon::new(&mock_server.server.url("/"))?
        .get("empty")
        .assert()
        .await
        .headers(contains(Vec::<(http::HeaderName, http::HeaderValue)>::new()));

    mock.assert();

    Ok(())
}

#[tokio::test]
async fn invalid_request_headers() -> Result<()> {
    let mock_server = HttpMockServer::new();
    let mock = mock_server.delete_valid_user();

    let grillon = Grillon::new(mock_server.server.url("/users").as_ref())?
        .log_settings(grillon::LogSettings::StdOutput);

    grillon
        .http_request(Method::DELETE, "users/1")
        .headers(vec![("ééç", "header value")])
        .assert()
        .await
        .headers(is(vec![("ééç", "header value")]))
        .status(is(204));

    mock.assert_hits(0);

    Ok(())
}
