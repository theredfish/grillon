# Requests

## HTTP

With Grillon, you can easily chain calls to configure and send HTTP requests and reuse the same
client for a given base api URL.

```rust
#[tokio::test]
async fn test_get_jsonplaceholder() -> Result<()> {
    let grillon = Grillon::new("https://jsonplaceholder.typicode.com")?;

    grillon
        .get("posts?id=1")
        .assert()
        .await
        .json_path("$[0].id", is(json!(1)));

    grillon
        .get("posts?id=2")
        .assert()
        .await
        .json_path("$[0].id", is(json!(2)));

    Ok(())
}
```

### Methods

Each method in this list has its corresponding `lowercase` function:

- GET
- POST
- PUT
- PATCH
- DELETE
- OPTIONS
- CONNECT
- HEAD

### Headers

Grillon supports two different types to configuring http request headers:

- `HeaderMap`
- `Vec<(HeaderName, HeaderValue)>`

```rust
let grillon = Grillon::new("https://jsonplaceholder.typicode.com")?;

// Vec<(HeaderName, HeaderValue)>
let request = grillon
    .post("posts")
    .payload(json!({
        "title": "foo",
        "body": "bar",
        "userId": 1
    }))
    .headers(vec![(
        CONTENT_TYPE,
        HeaderValue::from_static("application/json"),
    )]);

// Override with HeaderMap
let mut header_map = HeaderMap::new();
header_map.insert(CONTENT_TYPE, HeaderValue::from_static("application/json"));
let request = request.headers(header_map);
```

### Payload

At the moment, Grillon only supports the `application/json` content type. It will then be extended
with different content types such as `multipart/form-data`, `application/x-www-form-urlencoded`,
`text/plain` or `text/html`.

#### Json

Grillon re-exports `serde_json::Value` type to make it easier to add a json body. You can also use
the `json!` macro.

```rust
Grillon::new("https://jsonplaceholder.typicode.com")?;
    .post("posts")
    .payload(json!({
        "title": "foo",
        "body": "bar",
        "userId": 1
    }))
    .assert()
    .await;
```

### Build a custom request

If for some reasons you need a more programmatic way to create your http requests, you can use the
`http_request` function:

```rust
Grillon::new("https://jsonplaceholder.typicode.com")?
    .http_request(Method::POST, "posts")
    .assert()
    .await
    .status(is_success());
```
