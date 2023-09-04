# Assertions

Grillon provides domain-specific language per protocol and framework to make it natural to write
assertions.

An assertion is made up of:

- A part under test like `status`,
- a predicate such as `is`, `is_not`,
- and an expected value, for example `200`.

The predicates of a specific part can handle different type parameters. For example if you want to
assert that a status code is 200, you can pass a `u16` or a `StatusCode`. This information is
described below in the types column.

## Execution order

Your assertions are executed sequentially and in a blocking fashion. Asynchronous executions are not
supported yet. With sequential runs, order matters, so if you want to fail early under specific
conditions, it's possible. Each assertion produces [logs](../logs.md).

## HTTP assertion table

| part        | predicates                           | types                                   |
|:------------|:-------------------------------------|:----------------------------------------|
|headers      |is, is_not, contains, does_not_contain|Vec<(HeaderName, HeaderValue)>, HeaderMap|
|status       |is, is_not, is_between                |u16, StatusCode                          |
|json_body    |is, is_not, schema                    |String, &str, Value, `json!`             |
|json_path    |is, is_not, schema                    |Value, `json!`                           |
|response_time|is_less_than                          |u64                                      |

### Note about `json_path`

Json path requires one more argument than other predicates because you have to provide a path. The
expected value should always be a json document. To enforce this, we require a `Value` that you can
build by using the `json!` macro.

Here is an example of a json path assertion, where we are testing the value under the path
`$[0].id`.

```rust
#[tokio::test]
async fn test_json_path() -> Result<()> {
    Grillon::new("https://jsonplaceholder.typicode.com")?
        .get("posts?id=1")
        .assert()
        .await
        .json_path("$[0].id", is(json!(1)));

    Ok(())
}
```

### Note about `schema`

The `schema` predicate can be used with both `json_body` and `json_path` parts. Although the
available types differ between them, the `schema` predicate signature is independent and can be used
with the following types:

- &str
- String
- Value (you can also use the `json!` macro)

## Custom assertions

You may need to create more complex assertions or have more control on what is executed as part
of an assertion. If so, the library provides a specific function, `assert_fn`, allowing you to write
your own logic.

```rust
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
    .assert_fn(|assert| {
        assert!(!assert.headers.is_empty());
        assert!(assert.status == StatusCode::CREATED);
        assert!(assert.json.is_some());

        println!("Json response : {:#?}", assert.json);
    });
```

With this function you can access the `Assert` structure which is the internal representation of an
http response under test. You should have access to all parts that Grillon supports (headers, status, json, etc.). It's also
possible to add your own stdout logs if you want more control over the results or need to debug
what you receive.
