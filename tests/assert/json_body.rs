use crate::HttpMockServer;
use grillon::{
    dsl::{is, is_not},
    header::{HeaderValue, CONTENT_TYPE},
    json, Grillon, Result,
};

#[tokio::test]
async fn json_body() -> Result<()> {
    let mock_server = HttpMockServer::new();
    let mock = mock_server.get_valid_user();
    let json_header_map = vec![(CONTENT_TYPE, HeaderValue::from_static("application/json"))];

    Grillon::new(&mock_server.server.url("/"))?
        .get("users/1")
        .headers(json_header_map)
        .assert()
        .await
        .json_body(is(json!({
            "id": 1,
            "name": "Isaac",
        })));

    mock.assert();

    Ok(())
}

#[tokio::test]
async fn raw_string_body() -> Result<()> {
    let mock_server = HttpMockServer::new();
    let mock = mock_server.get_valid_user();
    let json_header_map = vec![(CONTENT_TYPE, HeaderValue::from_static("application/json"))];

    Grillon::new(&mock_server.server.url("/"))?
        .get("users/1")
        .headers(json_header_map)
        .assert()
        .await
        .json_body(is(r#"
        {
            "id": 1,
            "name": "Isaac"
        }
        "#));

    mock.assert();

    Ok(())
}

#[tokio::test]
async fn string_body() -> Result<()> {
    let mock_server = HttpMockServer::new();
    let mock = mock_server.get_valid_user();
    let json_header_map = vec![(CONTENT_TYPE, HeaderValue::from_static("application/json"))];
    let json = r#"
    {
        "id": 1,
        "name": "Isaac"
    }
    "#
    .to_string();

    Grillon::new(&mock_server.server.url("/"))?
        .get("users/1")
        .headers(json_header_map)
        .assert()
        .await
        .json_body(is(json));

    mock.assert();

    Ok(())
}

#[tokio::test]
async fn it_should_not_be_equals() {
    let mock_server = HttpMockServer::new();
    mock_server.get_valid_user();

    Grillon::new(&mock_server.server.url("/"))
        .unwrap()
        .get("users/1")
        .assert()
        .await
        .json_body(is_not(json!({
            "id": 101,
            "name": "Ecbert",
        })));
}

#[tokio::test]
#[should_panic]
async fn it_should_fail_to_compare_bad_body() {
    let mock_server = HttpMockServer::new();
    mock_server.get_valid_user();

    Grillon::new(&mock_server.server.url("/"))
        .unwrap()
        .get("users/1")
        .assert()
        .await
        .json_body(is(json!({
            "id": 100,
            "name": "Tom",
        })));
}

#[tokio::test]
#[should_panic]
async fn it_should_fail_to_compare_inexistant_body() {
    let mock_server = HttpMockServer::new();
    mock_server.get_empty_response();

    Grillon::new(&mock_server.server.url("/"))
        .unwrap()
        .get("empty")
        .assert()
        .await
        .json_body(is(json!({
            "id": 1,
            "name": "Isaac",
        })));
}
