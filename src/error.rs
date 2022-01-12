use serde::Deserialize;
use thiserror::Error;

/// Sift result type
pub type Result<T> = std::result::Result<T, Error>;

/// Sift errors
#[derive(Error, Debug, Deserialize)]
#[serde(untagged)]
pub enum Error {
    /// General client errors
    #[error("Sift auth error({error}): {description}")]
    Client {
        /// Error status message
        ///
        /// e.g. Forbidden
        error: String,

        /// Error description
        ///
        /// e.g. Permission denied
        description: String,

        /// Request issues
        #[serde(default)]
        issues: Option<serde_json::Value>,
    },

    /// Request errors
    #[error("Sift error ({status}): {error_message}")]
    Request {
        /// Non-zero indicates error status
        ///
        /// Docs <https://sift.com/developers/docs/curl/events-api/error-codes>
        status: i32,

        /// Error message
        ///
        /// e.g. Invalid API Key. Please check your credentials and try again.
        error_message: String,
    },

    /// Server errors
    #[error("Sift server error: {0}")]
    Server(String),
}

impl From<serde_json::Error> for Error {
    fn from(err: serde_json::Error) -> Self {
        Error::Server(err.to_string())
    }
}
