# Logs

Grillon provides a `LogSettings` structure so you can easily configure how the assertion results
should be output. The default log settings are set to `StdAssert`. Only failures will be printed
to standard output in a human-readable format.

Each assertion results in a log to standard output that you can connect with your infrastructure to
react to specific events. We could imagine for example an integration with CloudWatch and create an
alert as soon as the json log contains the key/value `"result": "failure"`.

## Human readable

### Failures only

This is the default, fail-fast, mode. As soon as you get a failure, the execution halts.

```rust
Grillon::new("http://jsonplaceholder.typicode.com")?
    .log_settings(LogSettings::StdAssert)
    .get("posts?id=1")
    .assert()
    .await
    .status(is_client_error());
```

As the status isn't a client error but a successful code, the assertion fails. The following logs
will be printed on the standard output:

```bash
part: status code
should be between: "400 and 499"
was: "200"
```

If you replace `is_client_error()` by `is_success()` you should now see a successful test without
any logs.

### Failures and successes

Now, if you want to log everything, even passing test cases (when debugging for example), then you
just need to change your log settings to `StdOut`:

```rust
Grillon::new("http://jsonplaceholder.typicode.com")?
    .log_settings(LogSettings::StdAssert)
    .get("posts?id=1")
    .assert()
    .await
    .status(is_success());
```

Which should produce similar output:

```bash
running 1 test

part: status code
should be between: "200 and 299"
test http::basic_http::test ... ok
```

## Json

The json format is to be used when you want to integrate external tools: CI/CD, logging services
such as Elasticsearch or Cloudwatch, reporting tools, etc.

```rust
Grillon::new("http://jsonplaceholder.typicode.com")?
    .log_settings(LogSettings::Json)
    .get("posts?id=1")
    .assert()
    .await
    .status(is_client_error());
```

With the previous code block, we get an assertion failure since the status code isn't a client
error. Here is the resulting json output (stdout) of the run:

```json
{
   "left":200,
   "part":"status code",
   "predicate":"should be between",
   "result":"failed",
   "right":[
      400,
      499
   ]
}
```

Grillon doesn't provide any connectors yet, so you will need to redirect stdout logs to a driver if
you want to ingest json logs with other services.
