use crate::HttpMockServer;
use grillon::{
    dsl::{contains, does_not_contain, does_not_match, is, is_not, matches},
    json, Grillon, Result,
};

#[tokio::test]
async fn json_path_should_be_equal_to_json() -> Result<()> {
    let mock_server = HttpMockServer::new();
    let mock = mock_server.get_valid_user();

    Grillon::new(&mock_server.server.url("/"))?
        .get("users/1")
        .assert()
        .await
        .json_path("$.id", is(json!(1)))
        .json_path("$.id", is("1"));

    mock.assert();

    Ok(())
}

#[tokio::test]
#[should_panic]
async fn json_path_should_be_equal_failure_bad_data() {
    let mock_server = HttpMockServer::new();
    mock_server.get_valid_user();

    let expected_json = json!({
        "id": 1,
        "name": "Max",
    });

    Grillon::new(&mock_server.server.url("/"))
        .unwrap()
        .get("users/1")
        .assert()
        .await
        .json_path("$", is(expected_json));
}

#[tokio::test]
#[should_panic]
async fn json_path_should_be_equal_failure_no_data() {
    let mock_server = HttpMockServer::new();
    mock_server.get_valid_user();

    let expected_json = json!({
        "id": 1,
        "name": "Isaac",
    });

    Grillon::new(&mock_server.server.url("/"))
        .unwrap()
        .get("users/1")
        .assert()
        .await
        .json_path("$.lastname", is(expected_json));
}

#[tokio::test]
async fn json_path_should_not_be_equal() -> Result<()> {
    let mock_server = HttpMockServer::new();
    let mock = mock_server.get_valid_user();

    let json = json!({
        "id": 2,
        "name": "Max",
    });

    Grillon::new(&mock_server.server.url("/"))?
        .get("users/1")
        .assert()
        .await
        .json_path("$", is_not(json));

    mock.assert();

    Ok(())
}

#[tokio::test]
async fn json_path_contains() -> Result<()> {
    let mock_server = HttpMockServer::new();
    let mock = mock_server.get_valid_user();

    let json = json!({
        "id": 1,
        "name": "Isaac",
    });
    let raw_json = r#"{
        "id": 1,
        "name": "Isaac"
    }"#;

    Grillon::new(&mock_server.server.url("/"))?
        .get("users/1")
        .assert()
        .await
        .json_path("$", contains(json))
        .json_path("$", contains(raw_json))
        .json_path("$.id", contains("1"));

    mock.assert();

    Ok(())
}

#[tokio::test]
async fn json_path_does_not_contain() -> Result<()> {
    let mock_server = HttpMockServer::new();
    let mock = mock_server.get_valid_user();

    let json = r#"{
        "id": 2,
        "name": "Unknown"
    }"#;

    Grillon::new(&mock_server.server.url("/"))?
        .get("users/1")
        .assert()
        .await
        .json_path("$", does_not_contain(json));

    mock.assert();

    Ok(())
}

#[tokio::test]
async fn json_path_matches() -> Result<()> {
    let mock_server = HttpMockServer::new();
    let mock = mock_server.get_valid_user();

    Grillon::new(&mock_server.server.url("/"))?
        .get("users/1")
        .assert()
        .await
        .json_path("$.name", matches("Isa+c"))
        .json_path("$.name", matches(r"Isa[a-z]{2}"))
        .json_path("$.name", matches("Isaac"));

    mock.assert();

    Ok(())
}

#[tokio::test]
async fn json_path_does_not_match() -> Result<()> {
    let mock_server = HttpMockServer::new();
    let mock = mock_server.get_valid_user();

    Grillon::new(&mock_server.server.url("/"))?
        .get("users/1")
        .assert()
        .await
        .json_path("$.name", does_not_match("^Isa$"));

    mock.assert();

    Ok(())
}

#[tokio::test]
#[should_panic]
async fn json_path_with_invalid_regex_pattern() {
    let mock_server: HttpMockServer = HttpMockServer::new();
    let mock = mock_server.get_valid_user();

    Grillon::new(&mock_server.server.url("/"))
        .unwrap()
        .get("users/1")
        .assert()
        .await
        .json_path("$.name", does_not_match(r"\"));

    mock.assert_hits(0);
}

#[tokio::test]
#[should_panic]
async fn json_path_regex_fails_with_null_value() {
    let mock_server: HttpMockServer = HttpMockServer::new();
    let mock = mock_server.get_valid_user();

    Grillon::new(&mock_server.server.url("/"))
        .unwrap()
        .get("users/1")
        .assert()
        .await
        // note the importance of the double quotes to get a valid json `Value`
        .json_path("$.unknown", matches(r#""Isaac""#));

    mock.assert_hits(0);
}
