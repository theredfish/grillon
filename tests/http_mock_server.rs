use httpmock::prelude::*;
use httpmock::{
    Method::{CONNECT, HEAD, PATCH},
    Mock, MockServer,
};
use serde_json::json;

pub struct HttpMockServer {
    pub server: httpmock::MockServer,
}

impl Default for HttpMockServer {
    fn default() -> Self {
        Self::new()
    }
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
            then.status(200)
                .header("content-type", "application/json")
                .header("date", "today")
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

    pub fn put_valid_user(&self) -> Mock {
        self.server.mock(|when, then| {
            when.method(PUT)
                .path("/users/1")
                .header("content-type", "application/json")
                .json_body(json!({ "name": "Isaac" }));

            then.status(204).header("content-location", "/users/1");
        })
    }

    pub fn delete_valid_user(&self) -> Mock {
        self.server.mock(|when, then| {
            when.method(DELETE).path("/users/1");
            then.status(204);
        })
    }

    pub fn patch_valid_user(&self) -> Mock {
        self.server.mock(|when, then| {
            when.method(PATCH)
                .header("content-type", "application/json-patch+json")
                .path("/users/1")
                .json_body(json!(
                    [
                        { "op": "replace", "path": "/name", "value": "Isaac 👣" }
                    ]
                ));
            then.status(204).header("content-location", "/users/1");
        })
    }

    pub fn options(&self) -> Mock {
        self.server.mock(|when, then| {
            when.method(OPTIONS).path("/");
            then.status(204).header(
                "access-control-allow-methods",
                "OPTIONS, GET, HEAD, POST, PUT, DELETE, PATCH",
            );
        })
    }

    pub fn head(&self) -> Mock {
        self.server.mock(|when, then| {
            when.method(HEAD).path("/movies/1");
            then.status(204).header("content-length", "91750400");
        })
    }

    pub fn connect(&self) -> Mock {
        self.server.mock(|when, then| {
            when.method(CONNECT).header(
                "user-agent",
                "Mozilla/5.0 (Windows NT 6.1; WOW64; Trident/7.0; rv:11.0) like Gecko",
            );
            then.status(200).header("proxy-agent", "Netscape-Proxy/1.1");
        })
    }

    pub fn get_empty_response(&self) -> Mock {
        self.server.mock(|when, then| {
            when.method(GET).path("/empty");
            then.status(200);
        })
    }

    pub fn server_error(&self) -> Mock {
        self.server.mock(|when, then| {
            when.method(GET).path("/server/error");
            then.status(500);
        })
    }

    pub fn auth(&self) -> Mock {
        self.server.mock(|when, then| {
            when.method(POST).path("/auth");
            then.status(200)
                .header("Set-Cookie", "SESSIONID=123; HttpOnly");
        })
    }

    pub fn authenticated_request(&self) -> Mock {
        self.server.mock(|when, then| {
            when.method(GET)
                .path("/authenticated/endpoint")
                .cookie("SESSIONID", "123");
            then.status(200);
        })
    }
}
