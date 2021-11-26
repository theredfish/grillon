use crate::HttpMockServer;
use grillon::{
    header::{HeaderValue, CONTENT_TYPE},
    json, Error, Grillon, StatusCode,
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
