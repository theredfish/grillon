use crate::error::Result;
use crate::Request;
use http::{HeaderMap, Method};
use reqwest::{Client, ClientBuilder};
use url::Url;

/// Top-level instance to configure a REST API http client.
///
/// [`Grillon`] provides everything to configure a REST API http client,
/// and initiate a [`Request`].
pub struct Grillon {
    base_url: Url,
    client: Client,
    log_settings: LogSettings,
}

/// The log settings to output test results.
///
/// The default configuration is `StdAssert`.
#[derive(Clone)]
pub enum LogSettings {
    /// Only prints assertion failures through `std::assert` macro.
    StdAssert,
    /// Prints all assertion results to the standard output.
    StdOutput,
    /// Formats assertion results into a json output.
    JsonOutput,
}

impl Default for LogSettings {
    fn default() -> Self {
        Self::StdAssert
    }
}

impl Grillon {
    /// Creates a new instance of `Grillon` with the base API url.
    ///
    /// # Example
    ///
    /// ```rust
    /// # use grillon::{Grillon, Result};
    /// # fn run() -> Result<()> {
    /// let grillon = Grillon::new("https://jsonplaceholder.typicode.com")?;
    /// # Ok(())
    /// # }
    /// ```
    ///
    /// # Errors
    ///
    /// This function fails if the supplied base url cannot be parsed as a [`Url`].
    pub fn new(base_url: &str) -> Result<Grillon> {
        let client = ClientBuilder::new().cookie_store(false).build()?;

        Ok(Grillon {
            base_url: base_url.parse::<Url>()?,
            client,
            log_settings: LogSettings::default(),
        })
    }

    /// Configure the logs to print the test results. By default the
    /// [`LogSettings`] are configured to output with the test library
    /// assertions on the standard output with [`LogSettings::StdAssert`].
    /// Only test failures will be printed.
    pub fn log_settings(mut self, log_settings: LogSettings) -> Self {
        self.log_settings = log_settings;

        self
    }

    /// Enable a persistent cookie store for the client. By default,
    /// no cookie store is used. Enabling the cookie store with `store_cookies()`
    /// will update the http client and set the store to a default implementation.
    pub fn store_cookies(mut self, enable: bool) -> Result<Grillon> {
        let client = ClientBuilder::new().cookie_store(enable).build()?;
        self.client = client;

        Ok(self)
    }

    /// Creates a new [`Request`] initialized with a `GET` method and the given path.
    ///
    /// # Example
    ///
    /// ```rust
    /// # use grillon::{Grillon, Result};
    /// # fn run() -> Result<()> {
    /// let request = Grillon::new("https://jsonplaceholder.typicode.com")?
    ///     .get("users");
    /// # Ok(())
    /// # }
    /// ```
    pub fn get(&self, path: &str) -> Request<'_> {
        self.http_request(Method::GET, path)
    }

    /// Creates a new [`Request`] initialized with a `POST` method and the given path.
    ///
    /// # Example
    ///
    /// ```rust
    /// # use grillon::{Grillon, Result};
    /// # fn run() -> Result<()> {
    /// let request = Grillon::new("https://jsonplaceholder.typicode.com")?
    ///     .post("users");
    /// # Ok(())
    /// # }
    /// ```
    pub fn post(&self, path: &str) -> Request<'_> {
        self.http_request(Method::POST, path)
    }

    /// Creates a new [`Request`] initialized with a `PUT` method and the given path.
    ///
    /// # Example
    ///
    /// ```rust
    /// # use grillon::{Grillon, Result};
    /// # fn run() -> Result<()> {
    /// let request = Grillon::new("https://jsonplaceholder.typicode.com")?
    ///     .put("users/1");
    /// # Ok(())
    /// # }
    /// ```
    pub fn put(&self, path: &str) -> Request<'_> {
        self.http_request(Method::PUT, path)
    }

    /// Creates a new [`Request`] initialized with a `PATCH` method and the given path.
    ///
    /// # Example
    ///
    /// ```rust
    /// # use grillon::{Grillon, Result};
    /// # fn run() -> Result<()> {
    /// let request = Grillon::new("https://jsonplaceholder.typicode.com")?
    ///     .patch("users/1");
    /// # Ok(())
    /// # }
    /// ```
    pub fn patch(&self, path: &str) -> Request<'_> {
        self.http_request(Method::PATCH, path)
    }

    /// Creates a new [`Request`] initialized with a `DELETE` method and the given path.
    ///
    /// # Example
    ///
    /// ```rust
    /// # use grillon::{Grillon, Result};
    /// # fn run() -> Result<()> {
    /// let request = Grillon::new("https://jsonplaceholder.typicode.com")?
    ///     .delete("users/1");
    /// # Ok(())
    /// # }
    /// ```
    pub fn delete(&self, path: &str) -> Request<'_> {
        self.http_request(Method::DELETE, path)
    }

    /// Creates a new [`Request`] initialized with an `OPTIONS` method and the given path.
    ///
    /// # Example
    ///
    /// ```rust
    /// # use grillon::{Grillon, Result, dsl::contains, header::{ACCESS_CONTROL_ALLOW_METHODS, HeaderValue}};
    /// # async fn run() -> Result<()> {
    /// Grillon::new("https://jsonplaceholder.typicode.com")?
    ///     .options("")
    ///     .assert()
    ///     .await
    ///     .headers(contains(vec![(
    ///         ACCESS_CONTROL_ALLOW_METHODS,
    ///         HeaderValue::from_static("GET,HEAD,PUT,PATCH,POST,DELETE"),
    ///     )]));
    /// # Ok(())
    /// # }
    /// ```
    pub fn options(&self, path: &str) -> Request<'_> {
        self.http_request(Method::OPTIONS, path)
    }

    /// Creates a new [`Request`] initialized with an `HEAD` method and the given path.
    ///
    /// # Example
    ///
    /// ```rust
    /// # use grillon::{Grillon, Result, dsl::contains, header::{CONTENT_LENGTH, HeaderValue}};
    /// # async fn run() -> Result<()> {
    /// Grillon::new("https://jsonplaceholder.typicode.com")?
    ///     .head("photos/1")
    ///     .assert()
    ///     .await
    ///     .headers(contains(vec![(CONTENT_LENGTH, HeaderValue::from_static("205"))]));
    /// # Ok(())
    /// # }
    /// ```
    pub fn head(&self, path: &str) -> Request<'_> {
        self.http_request(Method::HEAD, path)
    }

    /// Creates a new [`Request`] initialized with a `CONNECT` method and the given path.
    ///
    /// # Example
    ///
    /// ```rust
    /// # use grillon::{Grillon, Result};
    /// # fn run() -> Result<()> {
    /// let request = Grillon::new("http://home.netscape.com")?
    ///     .connect("");
    /// # Ok(())
    /// # }
    /// ```
    pub fn connect(&self, path: &str) -> Request<'_> {
        self.http_request(Method::CONNECT, path)
    }

    /// Create a new [`Request`] initialized with the given method and path.
    ///
    /// # Example
    ///
    /// ```rust
    /// # use grillon::{Grillon, Method, Result};
    /// # fn run() -> Result<()> {
    /// let request = Grillon::new("https://jsonplaceholder.typicode.com")?
    ///     .http_request(Method::GET, "users");
    /// # Ok(())
    /// # }
    /// ```
    pub fn http_request(&self, method: Method, path: &str) -> Request<'_> {
        let url = crate::url::concat(&self.base_url, path).unwrap_or_else(|err| panic!("{}", err));

        Request {
            method,
            url,
            headers: Ok(HeaderMap::new()),
            payload: None,
            client: &self.client,
            log_settings: &self.log_settings,
            basic_auth: None,
            bearer_auth: None,
        }
    }
}
