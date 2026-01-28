use async_trait::async_trait;

use crate::domain::models::{AuthToken, CookidooCredentials, DomainError};

/// Port for authentication operations.
///
/// Implementations of this trait handle the actual authentication
/// with external services (e.g., Cookidoo API).
#[async_trait]
pub trait AuthenticationService: Send + Sync {
    /// Authenticates with the given credentials and returns an auth token.
    ///
    /// # Errors
    /// Returns `DomainError::AuthenticationFailed` if authentication fails.
    async fn authenticate(
        &self,
        credentials: &CookidooCredentials,
    ) -> Result<AuthToken, DomainError>;

    /// Refreshes an expired token using the refresh token.
    ///
    /// # Errors
    /// Returns `DomainError::AuthenticationFailed` if refresh fails.
    async fn refresh_token(&self, refresh_token: &str) -> Result<AuthToken, DomainError>;
}