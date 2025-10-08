//! Kraken error

use reqwest::header::InvalidHeaderValue;
use thiserror::Error;

/// Kraken error
#[derive(Debug, Error)]
pub enum Error {
    /// Base64 error
    #[error(transparent)]
    Base64(#[from] base64::DecodeError),
    /// Reqwest error
    #[error(transparent)]
    Reqwest(#[from] reqwest::Error),
    /// Invalid header error
    #[error(transparent)]
    InvalidHeaderValue(#[from] InvalidHeaderValue),
    /// Serde query string error
    #[error(transparent)]
    SerdeQs(#[from] serde_qs::Error),
    /// Url error
    #[error(transparent)]
    Url(#[from] url::ParseError),
    /// Kraken response errors
    #[error("{0:?}")]
    Kraken(Vec<String>),
    /// Missing credentials
    #[error("missing credentials")]
    MissingCredentials,
    /// Missing result in response
    #[error("missing result")]
    MissingResult,
}
