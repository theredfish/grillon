use mantis::{
    header::{HeaderMap, HeaderName, HeaderValue, CONTENT_TYPE},
    json, Assert, Error, Mantis, Response, StatusCode,
};
use tokio_test::block_on;

mod basic_http;
mod http_mock_server;

#[test]
fn basic_get_request() -> Result<(), Error> {
    block_on(async {
        let mut expected_headers = HeaderMap::new();
        expected_headers.insert(
            CONTENT_TYPE,
            HeaderValue::from_static("application/json; charset=utf-8"),
        );
        expected_headers.insert(
            HeaderName::from_static("x-powered-by"),
            HeaderValue::from_static("Express"),
        );

        Mantis::new("http://jsonplaceholder.typicode.com")?
            .get("todos/1")
            .assert()
            .await
            .status_success()
            .status(StatusCode::OK)
            .body(json!({
                "completed": false,
                "id": 1,
                "title": "delectus aut autem",
                "userId": 1
            }))
            .body(
                r#"
            {
                "completed": false,
                "id": 1,
                "title": "delectus aut autem",
                "userId": 1
            }
            "#,
            )
            .body(
                r#"
            {
                "completed": false,
                "id": 1,
                "title": "delectus aut autem",
                "userId": 1
            }
            "#
                .to_string(),
            )
            .headers_eq(vec![
                (
                    CONTENT_TYPE,
                    HeaderValue::from_static("application/json; charset=utf-8"),
                ),
                (
                    HeaderName::from_static("x-powered-by"),
                    HeaderValue::from_static("Express"),
                ),
            ])
            .headers_eq(vec![])
            .headers_eq(expected_headers.clone())
            .headers_ne(vec![(
                CONTENT_TYPE,
                HeaderValue::from_static("text/html; charset=utf-8"),
            )]);

        Ok(())
    })
}

#[test]
fn reuse_client_for_multiple_tests() -> Result<(), Error> {
    block_on(async {
        let mantis = Mantis::new("http://jsonplaceholder.typicode.com")?;

        mantis
            .post("posts")
            .payload(json!({
                "title": "foo",
                "body": "bar",
                "userId": 1
            }))
            .assert()
            .await
            .status_success()
            .status(StatusCode::CREATED);

        // TODO : Handle trailing slashes
        mantis
            .get("todos/1")
            .assert()
            .await
            .status_success()
            .status(StatusCode::OK)
            .body(json!({
                "completed": false,
                "id": 1,
                "title": "delectus aut autem",
                "userId": 1
            }));

        Ok(())
    })
}

#[test]
fn use_custom_response_struct() -> Result<(), Error> {
    use serde_json::Value;
    use std::{future::Future, pin::Pin};

    block_on(async {
        struct MyResponse {
            status: u16,
            body: serde_json::Value,
            headers: HeaderMap,
        }

        let response = reqwest::get("http://jsonplaceholder.typicode.com/todos/1")
            .await
            .expect("Valid reqwest::Response");

        let status = response.status().as_u16();
        let headers = response.headers().clone();
        let body: Value = response.json().await.expect("Valid json");

        let response = MyResponse {
            status,
            headers,
            body,
        };

        impl Response for MyResponse {
            fn status(&self) -> StatusCode {
                StatusCode::from_u16(self.status).expect("Valid status code from u16")
            }

            fn json(self) -> Pin<Box<dyn Future<Output = Option<Value>> + Send>> {
                let json = async move { Some(self.body) };

                Box::pin(json)
            }

            fn headers(&self) -> HeaderMap {
                self.headers.clone()
            }
        }

        Assert::new(response).await.status_success();

        Ok(())
    })
}

#[test]
#[should_panic]
fn it_should_fail_to_compare_inexistant_body() {
    block_on(async {
        Mantis::new("http://jsonplaceholder.typicode.com")
            .unwrap()
            .delete("posts/1")
            .assert()
            .await
            .status_success()
            .status(StatusCode::OK)
            .body(json!({
                "id": 1,
            }));
    });
}
