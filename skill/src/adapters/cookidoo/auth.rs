use std::sync::Arc;
use std::time::Duration;

use async_trait::async_trait;
use tracing::{debug, error};

use crate::domain::models::{AuthToken, CookidooCredentials, DomainError};
use crate::domain::ports::AuthenticationService;

use super::client::CookidooClient;
use super::error::CookidooError;
use super::models::CookidooAuthResponse;
use super::token_cache::TokenCache;

/// OAuth token endpoint path.
const TOKEN_ENDPOINT: &str = "/ciam/auth/token";

/// Cookidoo authentication adapter implementing the AuthenticationService port.
pub struct CookidooAuthAdapter {
    client: CookidooClient,
    cache: Arc<TokenCache>,
    credentials: CookidooCredentials,
    auth_header: String,
}

impl CookidooAuthAdapter {
    /// Creates a new CookidooAuthAdapter.
    pub fn new(
        client: CookidooClient,
        credentials: CookidooCredentials,
        auth_header: String,
    ) -> Self {
        Self {
            client,
            cache: Arc::new(TokenCache::new()),
            credentials,
            auth_header,
        }
    }

    /// Creates a new CookidooAuthAdapter with a shared token cache.
    pub fn with_cache(
        client: CookidooClient,
        credentials: CookidooCredentials,
        auth_header: String,
        cache: Arc<TokenCache>,
    ) -> Self {
        Self {
            client,
            cache,
            credentials,
            auth_header,
        }
    }

    /// Returns a reference to the token cache.
    pub fn cache(&self) -> &Arc<TokenCache> {
        &self.cache
    }

    /// Gets a valid access token, refreshing or re-authenticating as needed.
    pub async fn get_valid_token(&self) -> Result<String, CookidooError> {
        // Check if we have a valid cached token
        if let Some(token) = self.cache.get() {
            if !token.needs_refresh() {
                debug!("Using cached token");
                return Ok(token.access_token().to_string());
            }

            // Try to refresh the token
            debug!("Token needs refresh, attempting refresh");
            match self.refresh_token_internal(token.refresh_token()).await {
                Ok(new_token) => {
                    let access = new_token.access_token().to_string();
                    self.cache.set(new_token);
                    return Ok(access);
                }
                Err(e) => {
                    debug!(error = %e, "Token refresh failed, will re-authenticate");
                    self.cache.clear();
                }
            }
        }

        // No valid token, perform full authentication
        debug!("Performing full authentication");
        let token = self.authenticate_internal(&self.credentials).await?;
        let access = token.access_token().to_string();
        self.cache.set(token);
        Ok(access)
    }

    async fn authenticate_internal(
        &self,
        credentials: &CookidooCredentials,
    ) -> Result<AuthToken, CookidooError> {
        let url = self.client.url(TOKEN_ENDPOINT);

        let params = [
            ("grant_type", "password"),
            ("username", credentials.email()),
            ("password", credentials.password()),
        ];

        let response = self
            .client
            .inner()
            .post(&url)
            .header("Authorization", &self.auth_header)
            .header("Content-Type", "application/x-www-form-urlencoded")
            .form(&params)
            .send()
            .await?;

        let status = response.status();

        if status.is_success() {
            let auth_response: CookidooAuthResponse = response
                .json()
                .await
                .map_err(|e| CookidooError::ParseError(e.to_string()))?;

            Ok(AuthToken::new(
                auth_response.access_token,
                auth_response.refresh_token,
                Duration::from_secs(auth_response.expires_in),
            ))
        } else if status.as_u16() == 401 {
            error!("Authentication failed: invalid credentials");
            Err(CookidooError::AuthenticationError(
                "Invalid credentials".to_string(),
            ))
        } else if status.as_u16() == 400 {
            let body = response.text().await.unwrap_or_default();
            error!(status = %status, body = %body, "Bad request during authentication");
            Err(CookidooError::BadRequest(body))
        } else {
            let body = response.text().await.unwrap_or_default();
            error!(status = %status, body = %body, "HTTP error during authentication");
            Err(CookidooError::HttpError {
                status: status.as_u16(),
                message: body,
            })
        }
    }

    async fn refresh_token_internal(&self, refresh_token: &str) -> Result<AuthToken, CookidooError> {
        let url = self.client.url(TOKEN_ENDPOINT);

        let params = [
            ("grant_type", "refresh_token"),
            ("refresh_token", refresh_token),
        ];

        let response = self
            .client
            .inner()
            .post(&url)
            .header("Authorization", &self.auth_header)
            .header("Content-Type", "application/x-www-form-urlencoded")
            .form(&params)
            .send()
            .await?;

        let status = response.status();

        if status.is_success() {
            let auth_response: CookidooAuthResponse = response
                .json()
                .await
                .map_err(|e| CookidooError::ParseError(e.to_string()))?;

            Ok(AuthToken::new(
                auth_response.access_token,
                auth_response.refresh_token,
                Duration::from_secs(auth_response.expires_in),
            ))
        } else {
            let body = response.text().await.unwrap_or_default();
            error!(status = %status, "Token refresh failed");
            Err(CookidooError::TokenExpired(format!(
                "Refresh failed with status {}: {}",
                status, body
            )))
        }
    }
}

#[async_trait]
impl AuthenticationService for CookidooAuthAdapter {
    async fn authenticate(
        &self,
        credentials: &CookidooCredentials,
    ) -> Result<AuthToken, DomainError> {
        self.authenticate_internal(credentials)
            .await
            .map_err(|e| e.into())
    }

    async fn refresh_token(&self, refresh_token: &str) -> Result<AuthToken, DomainError> {
        self.refresh_token_internal(refresh_token)
            .await
            .map_err(|e| e.into())
    }
}