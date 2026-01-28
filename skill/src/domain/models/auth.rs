use std::time::{Duration, Instant};

/// Credentials for authenticating with the Cookidoo API.
#[derive(Debug, Clone)]
pub struct CookidooCredentials {
    email: String,
    password: String,
}

impl CookidooCredentials {
    pub fn new(email: impl Into<String>, password: impl Into<String>) -> Self {
        Self {
            email: email.into(),
            password: password.into(),
        }
    }

    pub fn email(&self) -> &str {
        &self.email
    }

    pub fn password(&self) -> &str {
        &self.password
    }
}

/// Buffer time before actual expiry to trigger a refresh (5 minutes).
const REFRESH_BUFFER: Duration = Duration::from_secs(5 * 60);

/// Authentication token received from the Cookidoo API.
#[derive(Debug, Clone)]
pub struct AuthToken {
    access_token: String,
    refresh_token: String,
    expires_at: Instant,
}

impl AuthToken {
    /// Creates a new AuthToken with the given values.
    ///
    /// # Arguments
    /// * `access_token` - The access token for API calls
    /// * `refresh_token` - The refresh token for obtaining new access tokens
    /// * `expires_in` - Duration until the access token expires
    pub fn new(
        access_token: impl Into<String>,
        refresh_token: impl Into<String>,
        expires_in: Duration,
    ) -> Self {
        Self {
            access_token: access_token.into(),
            refresh_token: refresh_token.into(),
            expires_at: Instant::now() + expires_in,
        }
    }

    /// Returns the access token string.
    pub fn access_token(&self) -> &str {
        &self.access_token
    }

    /// Returns the refresh token string.
    pub fn refresh_token(&self) -> &str {
        &self.refresh_token
    }

    /// Returns true if the token has expired.
    pub fn is_expired(&self) -> bool {
        Instant::now() >= self.expires_at
    }

    /// Returns true if the token should be refreshed (within 5-minute buffer of expiry).
    pub fn needs_refresh(&self) -> bool {
        Instant::now() + REFRESH_BUFFER >= self.expires_at
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn credentials_stores_email_and_password() {
        let creds = CookidooCredentials::new("test@example.com", "secret123");
        assert_eq!(creds.email(), "test@example.com");
        assert_eq!(creds.password(), "secret123");
    }

    #[test]
    fn token_is_not_expired_when_fresh() {
        let token = AuthToken::new("access", "refresh", Duration::from_secs(3600));
        assert!(!token.is_expired());
    }

    #[test]
    fn token_is_expired_when_duration_is_zero() {
        let token = AuthToken::new("access", "refresh", Duration::ZERO);
        assert!(token.is_expired());
    }

    #[test]
    fn token_needs_refresh_within_buffer() {
        let token = AuthToken::new("access", "refresh", Duration::from_secs(4 * 60));
        assert!(token.needs_refresh());
    }

    #[test]
    fn token_does_not_need_refresh_when_fresh() {
        let token = AuthToken::new("access", "refresh", Duration::from_secs(3600));
        assert!(!token.needs_refresh());
    }

    #[test]
    fn token_accessors_return_correct_values() {
        let token = AuthToken::new("my_access", "my_refresh", Duration::from_secs(3600));
        assert_eq!(token.access_token(), "my_access");
        assert_eq!(token.refresh_token(), "my_refresh");
    }
}