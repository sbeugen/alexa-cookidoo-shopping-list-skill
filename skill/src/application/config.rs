use std::env;

use crate::domain::models::CookidooCredentials;

/// Environment variable names.
mod env_vars {
    pub const COOKIDOO_EMAIL: &str = "COOKIDOO_EMAIL";
    pub const COOKIDOO_PASSWORD: &str = "COOKIDOO_PASSWORD";
    pub const COOKIDOO_AUTH_HEADER: &str = "COOKIDOO_AUTH_HEADER";
}

/// Application configuration loaded from environment variables.
#[derive(Debug, Clone)]
pub struct AppConfig {
    cookidoo_credentials: CookidooCredentials,
    cookidoo_auth_header: String,
}

impl AppConfig {
    /// Loads configuration from environment variables.
    ///
    /// # Required Environment Variables
    /// - `COOKIDOO_EMAIL`: Cookidoo account email
    /// - `COOKIDOO_PASSWORD`: Cookidoo account password
    /// - `COOKIDOO_AUTH_HEADER`: Cookidoo API authorization header
    ///
    /// # Errors
    /// Returns an error if any required environment variable is missing.
    pub fn from_env() -> Result<Self, ConfigError> {
        let email = env::var(env_vars::COOKIDOO_EMAIL)
            .map_err(|_| ConfigError::MissingEnvVar(env_vars::COOKIDOO_EMAIL.to_string()))?;

        let password = env::var(env_vars::COOKIDOO_PASSWORD)
            .map_err(|_| ConfigError::MissingEnvVar(env_vars::COOKIDOO_PASSWORD.to_string()))?;

        let auth_header = env::var(env_vars::COOKIDOO_AUTH_HEADER)
            .map_err(|_| ConfigError::MissingEnvVar(env_vars::COOKIDOO_AUTH_HEADER.to_string()))?;

        Ok(Self {
            cookidoo_credentials: CookidooCredentials::new(email, password),
            cookidoo_auth_header: auth_header,
        })
    }

    /// Returns the Cookidoo credentials.
    pub fn cookidoo_credentials(&self) -> &CookidooCredentials {
        &self.cookidoo_credentials
    }

    /// Returns the Cookidoo API authorization header.
    pub fn cookidoo_auth_header(&self) -> &str {
        &self.cookidoo_auth_header
    }
}

/// Configuration errors.
#[derive(Debug, thiserror::Error)]
pub enum ConfigError {
    #[error("Missing required environment variable: {0}")]
    MissingEnvVar(String),
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::env;

    fn with_env_vars<F, R>(vars: &[(&str, &str)], f: F) -> R
    where
        F: FnOnce() -> R,
    {
        // Set vars
        for (key, value) in vars {
            env::set_var(key, value);
        }

        let result = f();

        // Clean up
        for (key, _) in vars {
            env::remove_var(key);
        }

        result
    }

    #[test]
    fn loads_config_from_env() {
        with_env_vars(
            &[
                ("COOKIDOO_EMAIL", "test@example.com"),
                ("COOKIDOO_PASSWORD", "secret123"),
                ("COOKIDOO_AUTH_HEADER", "Basic dGVzdDp0ZXN0"),
            ],
            || {
                let config = AppConfig::from_env().unwrap();
                assert_eq!(config.cookidoo_credentials().email(), "test@example.com");
                assert_eq!(config.cookidoo_credentials().password(), "secret123");
                assert_eq!(config.cookidoo_auth_header(), "Basic dGVzdDp0ZXN0");
            },
        );
    }

    #[test]
    fn returns_error_when_email_missing() {
        with_env_vars(
            &[
                ("COOKIDOO_PASSWORD", "secret123"),
                ("COOKIDOO_AUTH_HEADER", "Basic dGVzdDp0ZXN0"),
            ],
            || {
                let result = AppConfig::from_env();
                assert!(matches!(result, Err(ConfigError::MissingEnvVar(_))));
            },
        );
    }

    #[test]
    fn returns_error_when_password_missing() {
        with_env_vars(
            &[
                ("COOKIDOO_EMAIL", "test@example.com"),
                ("COOKIDOO_AUTH_HEADER", "Basic dGVzdDp0ZXN0"),
            ],
            || {
                let result = AppConfig::from_env();
                assert!(matches!(result, Err(ConfigError::MissingEnvVar(_))));
            },
        );
    }

    #[test]
    fn returns_error_when_auth_header_missing() {
        with_env_vars(
            &[
                ("COOKIDOO_EMAIL", "test@example.com"),
                ("COOKIDOO_PASSWORD", "secret123"),
            ],
            || {
                let result = AppConfig::from_env();
                assert!(matches!(result, Err(ConfigError::MissingEnvVar(_))));
            },
        );
    }
}