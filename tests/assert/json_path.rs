use crate::HttpMockServer;
use grillon::{
    dsl::{is, is_not},
    json, Grillon, Result,
};

#[tokio::test]
async fn json_path_should_be_equal_with_both_json_and_array() -> Result<()> {
    let mock_server = HttpMockServer::new();
    let mock = mock_server.get_valid_user();

    let path = "$";
    // This json value should automatically wrapped into an array by Grillon.
    // This is because the jsonpath-rust lib always encapsulate result into an
    // array since we can't know the number of items in advance.
    let expected_json = json!({
        "id": 1,
        "name": "Isaac",
    });
    let expected_json_array = json!([{
        "id": 1,
        "name": "Isaac",
    }]);

    Grillon::new(&mock_server.server.url("/"))?
        .get("users/1")
        .assert()
        .await
        .json_path(path, is(expected_json_array))
        .json_path(path, is(expected_json));

    mock.assert();

    Ok(())
}

#[tokio::test]
#[should_panic]
async fn json_path_should_be_equal_failure_bad_data() {
    let mock_server = HttpMockServer::new();
    mock_server.get_valid_user();

    let path = "$";
    let expected_json = json!({
        "id": 1,
        "name": "Max",
    });

    Grillon::new(&mock_server.server.url("/"))
        .unwrap()
        .get("users/1")
        .assert()
        .await
        .json_path(path, is(expected_json));
}

#[tokio::test]
#[should_panic]
async fn json_path_should_be_equal_failure_no_data() {
    let mock_server = HttpMockServer::new();
    mock_server.get_valid_user();

    let path = "$.lastname";
    let expected_json = json!({
        "id": 1,
        "name": "Isaac",
    });

    Grillon::new(&mock_server.server.url("/"))
        .unwrap()
        .get("users/1")
        .assert()
        .await
        .json_path(path, is(expected_json));
}

#[tokio::test]
async fn json_path_should_not_be_equal() -> Result<()> {
    let mock_server = HttpMockServer::new();
    let mock = mock_server.get_valid_user();

    let path = "$";
    let json = json!({
        "id": 2,
        "name": "Max",
    });

    Grillon::new(&mock_server.server.url("/"))?
        .get("users/1")
        .assert()
        .await
        .json_path(path, is_not(json));

    mock.assert();

    Ok(())
}
