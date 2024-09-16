use url::ParseError;

/// Short hand for `Result` type.
pub type Result<T> = std::result::Result<T, Error>;

/// Represents the library errors.
#[derive(thiserror::Error, Debug)]
pub enum Error {
    /// Url parse error.
    #[error("Invalid URL")]
    UrlParseError(#[from] ParseError),
}
