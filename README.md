# Grillon

Grillon offers an elegant and natural way to approach end-to-end HTTP API testing in Rust.

- Elegant, intuitive and expressive API
- Built-in testing functions
- Extensible

# Example

```rust
use grillon::{
    header::{HeaderValue, CONTENT_TYPE},
    json, Grillon, StatusCode,
};

#[tokio::main]
async fn main() {
    Grillon::new("http://jsonplaceholder.typicode.com")
        .unwrap()
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
}
```
