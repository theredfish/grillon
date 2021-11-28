use crate::HttpMockServer;
use grillon::{
    header::{HeaderValue, CONTENT_LOCATION, CONTENT_TYPE},
    json, Error, Grillon, Method, StatusCode,
};

#[tokio::test]
async fn post_request() -> Result<(), Error> {
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
async fn get_request() -> Result<(), Error> {
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
async fn put_request() -> Result<(), Error> {
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
async fn delete_request() -> Result<(), Error> {
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
async fn patch_request() -> Result<(), Error> {
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
async fn generic_request() -> Result<(), Error> {
    let mock_server = HttpMockServer::new();
    let mock = mock_server.delete_valid_user();

    Grillon::new(mock_server.server.url("/").as_ref())?
        .request(Method::DELETE, "users/1")
        .assert()
        .await
        .status_success()
        .status(StatusCode::NO_CONTENT);

    mock.assert();

    Ok(())
}
