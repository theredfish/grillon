# Quickstart

Using Grillon is pretty straightforward, we will consider you are running it as part of your testing
process. But you can also use it as a regular dependency.

## Configuration

Before we begin, let's create a `tests/` directory at the root of the project. Create a file there
named `create_posts.rs`.

Add `grillon` to your development dependencies with `tokio`, as we need a runtime to run async
functions in our test environement.

```toml
[dev-dependencies]
grillon = "0.5.0"
tokio = { version = "1", features = ["macros"] }
```

Our example will test the `/posts` endpoint of `jsonplaceholder.typicode.com`. We will send a json
payload and we will assert that our resource is correctly created with an acceptable response time
(< 500 ms). Depending on your location, feel free to tweak the response time value
(in milliseconds).

## Write the test

Create a new `create_posts.rs` file in `tests` and copy/paste the following example:

```rust,noplaypen
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

## Run the test

```bash
cargo test --test create_posts -- --nocapture
```

You should see similar output:

```bash
cargo test --test create_posts -- --nocapture
  Finished test [unoptimized + debuginfo] target(s) in 0.14s
    Running tests/create_posts.rs (target/debug/deps/create_posts-26c6ab07b039dabd)

running 1 test
Json response : Some(
    Object {
        "id": Number(101),
    },
)
test create_posts_monitoring ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.38s
```

Well done! You've written your first HTTP API test!

In this example, we performed assertions on:

- the status code
- the response time
- the headers
- the entire json body

We also added custom assertions and function calls with `assert_fn`. So if you have specific needs,
you can manipulate `assert` and add your own logic! For more information, you can read more about
[assertions](./writing_tests/assertions.md) in this book.

## Next steps

This book contains more in-depth content about Grillon such as reusing a request builder, how to
organize your tests, available assertions, and how to configure your log output. You can also find
technical information in our [latest API documentation](https://docs.rs/grillon/latest/grillon/).
