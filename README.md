# Grillon

[![Crates.io](https://img.shields.io/crates/v/grillon)](https://crates.io/crates/grillon)
[![docs.rs](https://img.shields.io/docsrs/grillon)](https://docs.rs/grillon/0.1.0/grillon/)
[![GitHub Workflow Status](https://img.shields.io/github/workflow/status/theredfish/grillon/Rust%20CI)](https://github.com/theredfish/grillon/actions?query=workflow%3A%22Rust+CI%22+branch%3Amain)

Grillon offers an elegant and natural way to approach end-to-end HTTP API testing in Rust.

- Elegant, intuitive and expressive API
- Built-in testing functions
- Extensible

> Please note that the API is subject to a lot of changes until the `v1.0.0`.

# Getting started

This example enables the optional `diff` feature and uses [Tokio](https://tokio.rs/) as asynchronous runtime.
Generally, testing libs are used in unit or integration tests. You can declare `grillon` as a dev-dependency.

Add `grillon` to `Cargo.toml`

```toml
[dev-dependencies]
grillon = { version = "0.1.0", features = ["diff"] }
tokio = { version = "1", features = ["macros"] }
```

Then use `grillon` :

```rust
use grillon::{
    header::{HeaderValue, CONTENT_TYPE},
    json, Grillon, StatusCode, Result
};

#[tokio::test]
async fn end_to_end_test() -> Result<()> {
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
