# Client configuration

Grillon can be configured in different ways. We use [Hyper](https://github.com/hyperium/hyper) as
the default HTTP client and provide you with a default configuration. By using Hyper, we can
leverage on the low-level API to inspect HTTP requests and responses and provide interesting
features to Grillon.

## Default client implementation

The default client implementation should provide you with the most common features. All you need to
do is configure the base API URL when you create an instance of Grillon.

```rust
let grillon = Grillon::new("http://jsonplaceholder.typicode.com")?;
```

This way you don't have to rewrite the base URL each time you want to send a request and perform
assertions on the response. You can reuse the existing client and create a new request. In the
following example we send a `POST` request to `http://jsonplaceholder.typicode.com/posts`:

```rust
let request = grillon
    .post("posts")
    .payload(json!({
        "title": "foo",
        "body": "bar",
        "userId": 1
    }))
    .assert()
    .await;
```

The `assert` function consumes the
[`grillon::Request`](https://docs.rs/grillon/latest/grillon/struct.Request.html) and prevents
further changes to the structure of the request when users want to run assertions on the response.

Refer to the [Requests](./requests.md) chapter for more information about how to configure your
requests. Note that at the moment Grillon only supports HTTP(s), but later we will extend the use
for different protocols and frameworks such as gRPC or SSL.

## Use a different client

When you want to use a different client to send your requests and handle the responses, you should
use the internal http response representation to assert. For that you need to use the
[`Assert`](https://docs.rs/grillon/latest/grillon/struct.Assert.html) structure.

For example, suppose you want to use `reqwest` to perform an http `GET` request and you want to
assert that the `response time` is `less than` 400ms. First you need to create your own structure
that will handle a `reqwest::Response`.

```rust
struct MyResponse {
    pub response: reqwest::Response,
}
```

Next, you need to implement the
[`grillon::Response`](https://docs.rs/grillon/latest/grillon/trait.Response.html) trait to describe
how you handle the various pieces of information that Grillon needs to perform assertions:

```rust
#[async_trait(?Send)]
impl Response for MyResponse {
    fn status(&self) -> StatusCode {
        self.response.status()
    }

    async fn json(self) -> Option<Value> {
        self.response.json::<Value>().await.ok()
    }

    fn headers(&self) -> HeaderMap {
        self.response.headers().clone()
    }
}
```

The next step is to create a new `Assert` instance which requires:

- An implementation of a `grillon::Response`,
- the response time in milliseconds,
- and the [`LogSettings`](https://docs.rs/grillon/latest/grillon/enum.LogSettings.html) for the
assertion results.

Let's first run the request and get the execution time:

```rust
let now = Instant::now();
let response = reqwest::get(mock_server.server.url("/users/1"))
    .await
    .expect("Failed to send the http request");
let response_time = now.elapsed().as_millis() as u64;
```

Now let's pass the response to your own response structure:

```rust
let my_response = MyResponse { response };
```

You are now ready to assert against a `reqwest::Response` wrapped by your own implementation of a
`grillon::Response`:

```rust
Assert::new(my_response, response_time, LogSettings::default())
    .await
    .response_time(is_less_than(400));
```
