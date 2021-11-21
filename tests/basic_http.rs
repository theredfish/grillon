use crate::http_mock_server::HttpMockServer;
use mantis::{
    header::{HeaderValue, CONTENT_TYPE},
    json, Error, Mantis, StatusCode,
};

#[tokio::test]
async fn post_request() -> Result<(), Error> {
    let mock_server = HttpMockServer::new();
    let mock = mock_server.post_valid_user();

    let json_header_map = vec![(CONTENT_TYPE, HeaderValue::from_static("application/json"))];

    Mantis::new(mock_server.server.url("/").as_ref())?
        .post("users")
        .payload(json!({
            "name": "Isaac",
        }))
        .headers(json_header_map.clone())
        .assert()
        .await
        .status_success()
        .status(StatusCode::CREATED)
        .headers_eq(json_header_map)
        .body(json!({
            "id": 1,
            "name": "Isaac"
        }));

    mock.assert();

    Ok(())
}
