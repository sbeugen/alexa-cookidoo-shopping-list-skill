use thiserror::Error;

/// Domain-level errors that can occur in the application.
/// These errors are technology-agnostic and represent business logic failures.
#[derive(Debug, Error)]
pub enum DomainError {
    /// The item name is invalid (empty or exceeds maximum length)
    #[error("Invalid item name: {0}")]
    InvalidItemName(String),

    /// Authentication with the external service failed
    #[error("Authentication failed: {0}")]
    AuthenticationFailed(String),

    /// A generic repository operation failed
    #[error("Repository error: {0}")]
    RepositoryError(String),
}