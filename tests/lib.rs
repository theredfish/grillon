use api_rs::{
    header::{HeaderMap, HeaderName, HeaderValue, CONTENT_TYPE},
    json, ApiHours, Error, StatusCode,
};
use tokio_test::block_on;

#[test]
fn basic_get_request() -> Result<(), Error> {
    block_on(async {
        let mut expected_headers = HeaderMap::new();
        expected_headers.insert(
            CONTENT_TYPE,
            HeaderValue::from_static("application/json; charset=utf-8"),
        );
        expected_headers.insert(
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
            .headers_eq(vec![
                (
                    CONTENT_TYPE,
                    HeaderValue::from_static("application/json; charset=utf-8"),
                ),
                (
                    HeaderName::from_static("x-powered-by"),
                    HeaderValue::from_static("Express"),
                ),
            ])
            .headers_eq(vec![])
            .headers_eq(expected_headers.clone())
            .headers_ne(vec![(
                CONTENT_TYPE,
                HeaderValue::from_static("text/html; charset=utf-8"),
            )]);

        Ok(())
    })
}

#[test]
fn basic_post_request() -> Result<(), Error> {
    block_on(async {
        ApiHours::new("http://jsonplaceholder.typicode.com")
            .post("/posts")
            .payload(json!({
                "title": "foo",
                "body": "bar",
                "userId": 1
            }))
            .verify()
            .await?
            .status_success()
            .status(StatusCode::CREATED)
            .headers_eq(vec![(
                CONTENT_TYPE,
                HeaderValue::from_static("application/json; charset=utf-8"),
            )])
            .body(json!({
                "id": 101,
            }));

        Ok(())
    })
}
