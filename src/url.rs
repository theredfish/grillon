use crate::error::Result;
use http::uri::Uri;

pub fn concat(base: &Uri, path: &str) -> Result<Uri> {
    format!("{}{}", base, path)
        .parse::<Uri>()
        .map_err(|err| err.into())
}
