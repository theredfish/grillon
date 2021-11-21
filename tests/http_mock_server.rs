use httpmock::prelude::*;
use httpmock::{Mock, MockServer};
use serde_json::json;

pub struct HttpMockServer {
    pub server: httpmock::MockServer,
}

impl HttpMockServer {
    pub fn new() -> Self {
        Self {
            server: MockServer::start(),
        }
    }

    pub fn get_valid_user(&self) -> Mock {
        self.server.mock(|when, then| {
            when.method(GET).path("/users/1");
            then.status(201)
                .header("content-type", "application/json")
                .json_body(json!({ "id": 1, "name": "Isaac" }));
        })
    }

    pub fn post_valid_user(&self) -> Mock {
        self.server.mock(|when, then| {
            when.method(POST)
                .path("/users")
                .header("content-type", "application/json")
                .json_body(json!({ "name": "Isaac" }));

            then.status(201)
                .header("content-type", "application/json")
                .json_body(json!({ "id": 1, "name": "Isaac" }));
        })
    }
}
