use crate::HttpMockServer;
use grillon::{
    header::{HeaderValue, CONTENT_TYPE},
    json, Grillon, Result,
};

#[tokio::test]
async fn json_body() -> Result<()> {
    let mock_server = HttpMockServer::new();
    let mock = mock_server.get_valid_user();
    let json_header_map = vec![(CONTENT_TYPE, HeaderValue::from_static("application/json"))];

    Grillon::new(mock_server.server.url("/").as_ref())?
        .get("users/1")
        .headers(json_header_map)
        .assert()
        .await
        .status_success()
        .body(json!({
            "id": 1,
            "name": "Isaac",
        }));

    mock.assert();

    Ok(())
}

#[tokio::test]
async fn raw_string_body() -> Result<()> {
    let mock_server = HttpMockServer::new();
    let mock = mock_server.get_valid_user();
    let json_header_map = vec![(CONTENT_TYPE, HeaderValue::from_static("application/json"))];

    Grillon::new(mock_server.server.url("/").as_ref())?
        .get("users/1")
        .headers(json_header_map)
        .assert()
        .await
        .status_success()
        .body(
            r#"
            {
                "id": 1,
                "name": "Isaac"
            }
            "#,
        );

    mock.assert();

    Ok(())
}

#[tokio::test]
async fn string_body() -> Result<()> {
    let mock_server = HttpMockServer::new();
    let mock = mock_server.get_valid_user();
    let json_header_map = vec![(CONTENT_TYPE, HeaderValue::from_static("application/json"))];

    Grillon::new(mock_server.server.url("/").as_ref())?
        .get("users/1")
        .headers(json_header_map)
        .assert()
        .await
        .status_success()
        .body(
            r#"
            {
                "id": 1,
                "name": "Isaac"
            }
            "#
            .to_string(),
        );

    mock.assert();

    Ok(())
}

#[tokio::test]
#[should_panic]
async fn it_should_fail_to_compare_bad_body() {
    let mock_server = HttpMockServer::new();
    mock_server.get_valid_user();

    Grillon::new(mock_server.server.url("/").as_ref())
        .unwrap()
        .get("users/1")
        .assert()
        .await
        .body(json!({
            "id": 100,
            "name": "Tom",
        }));
}

#[tokio::test]
#[should_panic]
async fn it_should_fail_to_compare_inexistant_body() {
    let mock_server = HttpMockServer::new();
    mock_server.get_empty_response();

    Grillon::new(mock_server.server.url("/").as_ref())
        .unwrap()
        .get("empty")
        .assert()
        .await
        .body(json!({
            "id": 1,
            "name": "Isaac",
        }));
}
