//! Kraken API responses

use serde::Deserialize;

use crate::error::Error;

#[derive(Debug, Deserialize)]
pub(crate) struct KrakenResult<T> {
    /// Kraken API returns error strings in an array marked "error"
    error: Vec<String>,
    /// Kraken API returns results here, separated from error
    /// Sometimes result is omitted if errors occured.
    result: Option<T>,
}

impl<T> KrakenResult<T> {
    pub(crate) fn extract(self) -> Result<T, Error> {
        if !self.error.is_empty() {
            return Err(Error::Kraken(self.error));
        }

        self.result.ok_or(Error::MissingResult)
    }
}
