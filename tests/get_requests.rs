use api_rs::{json, ApiHours, ApiHoursError, StatusCode};
use tokio_test::block_on;

#[test]
fn simple_positive_assert_get_request() -> Result<(), ApiHoursError> {
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
            }));

        Ok(())
    })
}
