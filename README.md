# Mantis

An elegant and natural way to approach API testing in Rust.

- Hyper as default http client
- Extensible
- Elegant and expressive API

## Current state

Mantis is in active development and will be subject to a lot of changes before a first stable version. For this reason
the crate isn't published yet.

# Example

You can use Mantis in your tests as a dev dependency. Here an example with `tokio_test`.

```rust
use tokio_test::block_on;
use mantis::{
    header::{HeaderValue, CONTENT_TYPE},
    json, Error, Mantis, StatusCode
};

#[test]
fn should_create_a_new_post() -> Result<(), Error> {
    block_on(async {
        Mantis::new("http://jsonplaceholder.typicode.com")?
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
```
