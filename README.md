# Grillon

[![Crates.io](https://img.shields.io/crates/v/grillon)](https://crates.io/crates/grillon)
[![docs.rs](https://img.shields.io/docsrs/grillon)](https://docs.rs/grillon/latest/grillon)
![GitHub Workflow Status](https://img.shields.io/github/actions/workflow/status/theredfish/grillon/ci.yml)
[![Check Links](https://github.com/theredfish/grillon/actions/workflows/links.yml/badge.svg)](https://github.com/theredfish/grillon/actions/workflows/links.yml)

Grillon offers an elegant and natural way to approach API testing in Rust.

- Elegant, intuitive and expressive API
- Built-in testing functions
- Extensible

> Please note that the API is subject to a lot of changes until the `v1.0.0`.

## Documentation

- Book ([current](https://theredfish.github.io/grillon/current) | [dev](https://theredfish.github.io/grillon/dev))
- [API doc](https://docs.rs/grillon/latest/grillon)
- [Changelog](https://github.com/theredfish/grillon/blob/main/CHANGELOG.md)

## Getting started

You need [Tokio](https://tokio.rs/) as asynchronous runtime. Generally, testing libs are
used in unit or integration tests so let's declare `grillon` as a dev-dependency.

Add `grillon` to `Cargo.toml`

```toml
[dev-dependencies]
grillon = "0.5.0"
tokio = { version = "1", features = ["macros"] }
```

Then use `grillon` :

```rust
use grillon::{dsl::*, dsl::http::*, json, Grillon, StatusCode, Result};
use grillon::header::{HeaderValue, CONTENT_LENGTH, CONTENT_TYPE};
use grillon::Assert;

#[tokio::test]
async fn end_to_end_test() -> Result<()> {
    Grillon::new("https://jsonplaceholder.typicode.com")?
        .post("posts")
        .payload(json!({
            "title": "foo",
            "body": "bar",
            "userId": 1
        }))
        .assert()
        .await
        .status(is_success())
        .status(is(201))
        .response_time(is_less_than(700))
        .json_body(is(json!({
            "id": 101,
        })))
        .json_body(schema(json!({
            "properties": {
                "id": { "type": "number" }
            }
        })))
        .json_path("$.id", is(json!(101)))
        .header(CONTENT_TYPE, is("application/json; charset=utf-8"))
        .headers(contains(vec![
            (
                CONTENT_TYPE,
                HeaderValue::from_static("application/json; charset=utf-8"),
            ),
            (CONTENT_LENGTH, HeaderValue::from_static("15")),
        ]))
        .assert_fn(|assert| {
            let Assert {
                headers,
                status,
                json,
                ..
            } = assert.clone();

            assert!(!headers.unwrap().is_empty());
            assert!(status.unwrap() == StatusCode::CREATED);
            assert!(json.is_some());

            println!("Json response : {:#?}", assert.json);
        });

    Ok(())
}

```
