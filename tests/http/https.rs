use grillon::{
    dsl::{contains, http::is_success},
    header::{HeaderName, HeaderValue},
    Grillon, Result,
};

// Test a real https call
#[tokio::test]
async fn create_posts_monitoring() -> Result<()> {
    Grillon::new("https://jsonplaceholder.typicode.com")?
        .options("")
        .assert()
        .await
        .status(is_success())
        .headers(contains(vec![(
            HeaderName::from_static("access-control-allow-methods"),
            HeaderValue::from_static("GET,HEAD,PUT,PATCH,POST,DELETE"),
        )]));

    Ok(())
}
