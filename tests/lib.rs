use api_rs::{
    header::{HeaderMap, HeaderName, HeaderValue, CONTENT_TYPE},
    json, ApiHours, Error, StatusCode,
};
use tokio_test::block_on;

#[test]
fn simple_positive_assert_get_request() -> Result<(), Error> {
    block_on(async {
        let mut expected_header_map = HeaderMap::new();
        expected_header_map.insert(
            CONTENT_TYPE,
            HeaderValue::from_static("application/json; charset=utf-8"),
        );
        expected_header_map.insert(
            HeaderName::from_static("x-powered-by"),
            HeaderValue::from_static("Express"),
        );

        ApiHours::new("http://jsonplaceholder.typicode.com")
            .get("/todos/1")
            .verify()
            .await?
            .status_success()
            .status(StatusCode::OK)
            .body(json!({
                "completed": false,
                "id": 1,
                "title": "delectus aut autem",
                "userId": 1
            }))
            .body(
                r#"
            {
                "completed": false,
                "id": 1,
                "title": "delectus aut autem",
                "userId": 1
            }
            "#,
            )
            .body(
                r#"
            {
                "completed": false,
                "id": 1,
                "title": "delectus aut autem",
                "userId": 1
            }
            "#
                .to_string(),
            )
            .has_headers(vec![
                (
                    CONTENT_TYPE,
                    HeaderValue::from_static("application/json; charset=utf-8"),
                ),
                (
                    HeaderName::from_static("x-powered-by"),
                    HeaderValue::from_static("Express"),
                ),
            ])
            .has_headers(vec![])
            .has_headers(expected_header_map);

        Ok(())
    })
}
