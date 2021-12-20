use crate::HttpMockServer;
use grillon::{
    header::{
        HeaderName, HeaderValue, ACCESS_CONTROL_ALLOW_METHODS, CONTENT_LENGTH, CONTENT_LOCATION,
        CONTENT_TYPE, USER_AGENT,
    },
    json, Grillon, Method, Result, StatusCode,
};

#[tokio::test]
async fn post_request() -> Result<()> {
    let mock_server = HttpMockServer::new();
    let mock = mock_server.post_valid_user();

    let json_header_map = vec![(CONTENT_TYPE, HeaderValue::from_static("application/json"))];

    Grillon::new(mock_server.server.url("/").as_ref())?
        .post("users")
        .payload(json!({
            "name": "Isaac",
        }))
        .headers(json_header_map.clone())
        .assert()
        .await
        .status_success()
        .status(StatusCode::CREATED)
        .headers_exist(json_header_map)
        .body(json!({
            "id": 1,
            "name": "Isaac"
        }));

    mock.assert();

    Ok(())
}

#[tokio::test]
async fn get_request() -> Result<()> {
    let mock_server = HttpMockServer::new();
    let mock = mock_server.get_valid_user();

    let json_header_map = vec![(CONTENT_TYPE, HeaderValue::from_static("application/json"))];

    Grillon::new(mock_server.server.url("/").as_ref())?
        .get("users/1")
        .headers(json_header_map.clone())
        .assert()
        .await
        .status_success()
        .status(StatusCode::OK)
        .headers_exist(json_header_map)
        .body(json!({
            "id": 1,
            "name": "Isaac"
        }));

    mock.assert();

    Ok(())
}

#[tokio::test]
async fn put_request() -> Result<()> {
    let mock_server = HttpMockServer::new();
    let mock = mock_server.put_valid_user();

    Grillon::new(mock_server.server.url("/").as_ref())?
        .put("users/1")
        .headers(vec![(
            CONTENT_TYPE,
            HeaderValue::from_static("application/json"),
        )])
        .payload(json!({
            "name": "Isaac",
        }))
        .assert()
        .await
        .status_success()
        .status(StatusCode::NO_CONTENT)
        .headers_exist(vec![(
            CONTENT_LOCATION,
            HeaderValue::from_static("/users/1"),
        )]);

    mock.assert();

    Ok(())
}

#[tokio::test]
async fn delete_request() -> Result<()> {
    let mock_server = HttpMockServer::new();
    let mock = mock_server.delete_valid_user();

    Grillon::new(mock_server.server.url("/").as_ref())?
        .delete("users/1")
        .assert()
        .await
        .status_success()
        .status(StatusCode::NO_CONTENT);

    mock.assert();

    Ok(())
}

#[tokio::test]
async fn patch_request() -> Result<()> {
    let mock_server = HttpMockServer::new();
    let mock = mock_server.patch_valid_user();

    Grillon::new(mock_server.server.url("/").as_ref())?
        .patch("users/1")
        .payload(json!(
            [
                { "op": "replace", "path": "/name", "value": "Isaac 👣" }
            ]
        ))
        .headers(vec![(
            CONTENT_TYPE,
            HeaderValue::from_static("application/json-patch+json"),
        )])
        .assert()
        .await
        .status_success()
        .status(StatusCode::NO_CONTENT)
        .headers_exist(vec![(
            CONTENT_LOCATION,
            HeaderValue::from_static("/users/1"),
        )]);

    mock.assert();

    Ok(())
}

#[tokio::test]
async fn options_request() -> Result<()> {
    let mock_server = HttpMockServer::new();
    let mock = mock_server.options();

    Grillon::new(mock_server.server.url("/").as_ref())?
        .options("")
        .assert()
        .await
        .status_success()
        .status(StatusCode::NO_CONTENT)
        .headers_exist(vec![(
            ACCESS_CONTROL_ALLOW_METHODS,
            HeaderValue::from_static("OPTIONS, GET, HEAD, POST, PUT, DELETE, PATCH"),
        )]);

    mock.assert();

    Ok(())
}

#[tokio::test]
async fn head_request() -> Result<()> {
    let mock_server = HttpMockServer::new();
    let mock = mock_server.head();

    Grillon::new(mock_server.server.url("/").as_ref())?
        .head("movies/1")
        .assert()
        .await
        .status_success()
        .status(StatusCode::NO_CONTENT)
        .headers_exist(vec![(CONTENT_LENGTH, HeaderValue::from_static("91750400"))]);

    mock.assert();

    Ok(())
}

#[tokio::test]
async fn connect_request() -> Result<()> {
    let mock_server = HttpMockServer::new();
    let mock = mock_server.connect();

    Grillon::new(mock_server.server.url("/").as_ref())?
        .connect("")
        .headers(vec![(
            USER_AGENT,
            HeaderValue::from_static(
                "Mozilla/5.0 (Windows NT 6.1; WOW64; Trident/7.0; rv:11.0) like Gecko",
            ),
        )])
        .assert()
        .await
        .status_success()
        .status(StatusCode::OK)
        .headers_exist(vec![(
            HeaderName::from_static("proxy-agent"),
            HeaderValue::from_static("Netscape-Proxy/1.1"),
        )]);

    mock.assert();

    Ok(())
}

#[tokio::test]
async fn generic_http_request() -> Result<()> {
    let mock_server = HttpMockServer::new();
    let mock = mock_server.delete_valid_user();

    Grillon::new(mock_server.server.url("/").as_ref())?
        .http_request(Method::DELETE, "users/1")
        .assert()
        .await
        .status_success()
        .status(StatusCode::NO_CONTENT);

    mock.assert();

    Ok(())
}
