use crate::error::Result;
use url::Url;

pub(crate) fn concat(base: &Url, path: &str) -> Result<Url> {
    format!("{}{}", base, path)
        .parse::<Url>()
        .map_err(|err| err.into())
}
