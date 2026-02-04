use thiserror::Error;

use crate::domain::models::DomainError;

/// Errors specific to the Cookidoo API adapter.
#[derive(Debug, Error)]
pub enum CookidooError {
    /// Network or HTTP request failed
    #[error("Request failed: {0}")]
    RequestError(String),

    /// Authentication failed (401, invalid credentials)
    #[error("Authentication failed: {0}")]
    AuthenticationError(String),

    /// Bad request format (400)
    #[error("Bad request: {0}")]
    BadRequest(String),

    /// Failed to parse JSON response
    #[error("Failed to parse response: {0}")]
    ParseError(String),

    /// HTTP error with status code
    #[error("HTTP error {status}: {message}")]
    HttpError { status: u16, message: String },

    /// Token has expired and refresh failed
    #[error("Token expired and refresh failed: {0}")]
    TokenExpired(String),
}

impl From<reqwest::Error> for CookidooError {
    fn from(err: reqwest::Error) -> Self {
        if err.is_timeout() {
            CookidooError::RequestError("Request timed out".to_string())
        } else if err.is_connect() {
            CookidooError::RequestError("Failed to connect".to_string())
        } else {
            CookidooError::RequestError(err.to_string())
        }
    }
}

impl From<CookidooError> for DomainError {
    fn from(err: CookidooError) -> Self {
        match err {
            CookidooError::AuthenticationError(msg) => DomainError::AuthenticationFailed(msg),
            CookidooError::TokenExpired(msg) => DomainError::AuthenticationFailed(msg),
            other => DomainError::RepositoryError(other.to_string()),
        }
    }
}
