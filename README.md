# Grillon

Grillon offers an elegant and natural way to approach end-to-end HTTP API testing in Rust.

- Elegant, intuitive and expressive API
- Built-in testing functions
- Extensible

# Example

This example enables the optional `diff` feature and uses [Tokio](https://tokio.rs/) as asynchronous runtime.
Generally, testing libs are used in unit or integration tests. You can declare `grillon` as a dev-dependency.

## Cargo

```toml
[dev-dependencies]
grillon = { version = "0.1.0", features = ["diff"] }
tokio = { version = "1", features = ["macros"] }
```

## Code

```rust
use grillon::{
    header::{HeaderValue, CONTENT_TYPE},
    json, Grillon, StatusCode, Error
};

#[tokio::test]
async fn end_to_end_test() -> Result<(), Error> {
    Grillon::new("http://jsonplaceholder.typicode.com")?
        .post("posts")
        .payload(json!({
            "title": "foo",
            "body": "bar",
            "userId": 1
        }))
        .assert()
        .await
        .status_success()
        .status(StatusCode::CREATED)
        .headers_exist(vec![(
            CONTENT_TYPE,
            HeaderValue::from_static("application/json; charset=utf-8"),
        )])
        .body(json!({
            "id": 101,
        }));

    Ok(())
}
```
