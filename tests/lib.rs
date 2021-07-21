use api_rs::{json, ApiHours, Error, StatusCode};
use tokio_test::block_on;

#[test]
fn simple_positive_assert_get_request() -> Result<(), Error> {
    block_on(async {
        println!("Start runner");
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
            );
        Ok(())
    })
}
