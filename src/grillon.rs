use crate::error::Result;
use crate::Request;
use hyper::{client::HttpConnector, header::HeaderMap, Client, Method, Uri};

/// Top-level instance to configure a REST API http client.
///
/// [`Grillon`] provides everything to configure a REST API http client,
/// and initiate a [`Request`].
pub struct Grillon {
    base_url: Uri,
    client: Client<HttpConnector>,
    log_settings: LogSettings,
}

/// The log settings to output test results.
///
/// The default configuration is `StdAssert`.
#[derive(Clone)]
pub enum LogSettings {
    /// Prints all assertion results to the standard output.
    StdOut,
    /// Only prints assertion failures through `std::assert` macro.
    StdAssert,
    /// Formats assertion results into a json output.
    Json,
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
    /// let grillon = Grillon::new("http://jsonplaceholder.typicode.com")?;
    /// # Ok(())
    /// # }
    /// ```
    ///
    /// # Errors
    ///
    /// This function fails if the supplied base url cannot be parsed as a [`Uri`].
    pub fn new(api_base_url: &str) -> Result<Grillon> {
        Ok(Grillon {
            base_url: api_base_url.parse::<Uri>()?,
            client: Client::builder().build_http(),
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

    /// Creates a new [`Request`] initialized with a `GET` method and the given path.
    ///
    /// # Example
    ///
    /// ```rust
    /// # use grillon::{Grillon, Result};
    /// # fn run() -> Result<()> {
    /// let request = Grillon::new("http://jsonplaceholder.typicode.com")?
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
    /// let request = Grillon::new("http://jsonplaceholder.typicode.com")?
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
    /// let request = Grillon::new("http://jsonplaceholder.typicode.com")?
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
    /// let request = Grillon::new("http://jsonplaceholder.typicode.com")?
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
    /// let request = Grillon::new("http://jsonplaceholder.typicode.com")?
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
    /// Grillon::new("http://jsonplaceholder.typicode.com")?
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
    /// Grillon::new("http://jsonplaceholder.typicode.com")?
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
    /// let request = Grillon::new("http://jsonplaceholder.typicode.com")?
    ///     .http_request(Method::GET, "users");
    /// # Ok(())
    /// # }
    /// ```
    pub fn http_request(&self, method: Method, path: &str) -> Request<'_> {
        let uri = crate::url::concat(&self.base_url, path).unwrap_or_else(|err| panic!("{}", err));

        Request {
            method,
            uri,
            headers: HeaderMap::new(),
            payload: None,
            client: &self.client,
            log_settings: &self.log_settings,
        }
    }
}
