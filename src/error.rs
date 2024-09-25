/// Short hand for `Result` type.
pub type Result<T> = std::result::Result<T, Error>;

/// Represents the library errors.
#[derive(thiserror::Error, Debug)]
pub enum Error {
    /// Url parse error.
    #[error("Invalid URL")]
    UrlParseError(#[from] url::ParseError),
    /// Http client error.
    #[error("Http client error")]
    HttpClientError(#[from] reqwest::Error),
    /// Invalid header name.
    #[error("Invalid header name")]
    InvalidHeaderName(#[from] reqwest::header::InvalidHeaderName),
    /// Invalid header value.
    #[error("Invalid header value")]
    InvalidHeaderValue(#[from] reqwest::header::InvalidHeaderValue),
}
