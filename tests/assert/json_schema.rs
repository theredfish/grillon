use crate::HttpMockServer;
use grillon::{
    dsl::{is, schema},
    json, Grillon, Result,
};

#[tokio::test]
async fn json_body_matches_schema() -> Result<()> {
    let mock_server: HttpMockServer = HttpMockServer::new();
    let mock = mock_server.get_valid_user();
    let json_schema = json!(
        {
            "type": "object",
            "properties": {
                "id": {
                    "type": "number",
                    "description": "the user ID"
                },
                "name": {
                    "type": "string",
                    "description": "the user's name"
                }
            },
            "required": ["id", "name"]
        }
    );

    Grillon::new(&mock_server.server.url("/"))?
        .get("users/1")
        .assert()
        .await
        .json_body(is(json!({
            "id": 1,
            "name": "Isaac",
        })))
        .json_body(schema(json_schema));

    mock.assert();

    Ok(())
}

#[tokio::test]
#[should_panic]
async fn json_body_does_not_match_schema() {
    let mock_server: HttpMockServer = HttpMockServer::new();
    mock_server.get_valid_user();
    let json_schema = json!(
        {
            "type": "object",
            "properties": {
                "id": {
                    "type": "number",
                    "description": "the user ID",
                },
                "age": {
                    "type": "number",
                    "description": "the user age",
                },
                "name": {
                    "type": "string",
                    "description": "the user's name"
                }
            },
            "required": ["id", "name", "age"]
        }
    );

    Grillon::new(&mock_server.server.url("/"))
        .unwrap()
        .get("users/1")
        .assert()
        .await
        .json_body(is(json!({
            "id": 1,
            "name": "Isaac",
        })))
        .json_body(schema(json_schema));
}

#[tokio::test]
async fn json_path_value_matches_schema() -> Result<()> {
    let mock_server: HttpMockServer = HttpMockServer::new();
    let mock = mock_server.get_valid_user();
    let json_schema = json!(
        {
            "type": "array",
            "maxItems": 1,
            "items": {
                "type": "number"
            },
            "description": "the user ID from the json path"
        }
    );

    Grillon::new(&mock_server.server.url("/"))?
        .get("users/1")
        .assert()
        .await
        .json_body(is(json!({
            "id": 1,
            "name": "Isaac",
        })))
        .json_path("$.id", schema(json_schema));

    mock.assert();

    Ok(())
}
