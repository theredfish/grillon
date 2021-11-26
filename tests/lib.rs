use grillon::{
    header::{HeaderValue, CONTENT_TYPE},
    json, Error, Grillon, StatusCode,
};
use http_mock_server::HttpMockServer;

mod assert;
mod http;
mod http_mock_server;

#[tokio::test]
async fn reuse_grillon_for_multiple_tests() -> Result<(), Error> {
    let mock_server = HttpMockServer::new();
    let mock_post = mock_server.post_valid_user();
    let mock_get = mock_server.get_valid_user();
    let headers = vec![(CONTENT_TYPE, HeaderValue::from_static("application/json"))];

    let grillon = Grillon::new(mock_server.server.url("/").as_ref())?;

    grillon
        .post("users")
        .payload(json!({
            "name": "Isaac",
        }))
        .headers(headers)
        .assert()
        .await
        .status(StatusCode::CREATED);

    mock_post.assert();

    grillon
        .get("users/1")
        .assert()
        .await
        .status(StatusCode::OK)
        .body(json!({
            "id": 1,
            "name": "Isaac",
        }));

    mock_get.assert();

    Ok(())
}
