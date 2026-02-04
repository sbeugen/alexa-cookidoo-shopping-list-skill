use std::env;

use crate::domain::models::CookidooCredentials;

/// Environment variable names.
mod env_vars {
    pub const COOKIDOO_EMAIL: &str = "COOKIDOO_EMAIL";
    pub const COOKIDOO_PASSWORD: &str = "COOKIDOO_PASSWORD";
    pub const COOKIDOO_CLIENT_ID: &str = "COOKIDOO_CLIENT_ID";
    pub const COOKIDOO_CLIENT_SECRET: &str = "COOKIDOO_CLIENT_SECRET";
}

/// Application configuration loaded from environment variables.
#[derive(Debug, Clone)]
pub struct AppConfig {
    cookidoo_credentials: CookidooCredentials,
    cookidoo_client_id: String,
    cookidoo_client_secret: String,
}

impl AppConfig {
    /// Loads configuration from environment variables.
    ///
    /// # Required Environment Variables
    /// - `COOKIDOO_EMAIL`: Cookidoo account email
    /// - `COOKIDOO_PASSWORD`: Cookidoo account password
    /// - `COOKIDOO_CLIENT_ID`: Cookidoo OAuth client ID
    /// - `COOKIDOO_CLIENT_SECRET`: Cookidoo OAuth client secret
    ///
    /// # Errors
    /// Returns an error if any required environment variable is missing.
    pub fn from_env() -> Result<Self, ConfigError> {
        let email = env::var(env_vars::COOKIDOO_EMAIL)
            .map_err(|_| ConfigError::MissingEnvVar(env_vars::COOKIDOO_EMAIL.to_string()))?;

        let password = env::var(env_vars::COOKIDOO_PASSWORD)
            .map_err(|_| ConfigError::MissingEnvVar(env_vars::COOKIDOO_PASSWORD.to_string()))?;

        let client_id = env::var(env_vars::COOKIDOO_CLIENT_ID)
            .map_err(|_| ConfigError::MissingEnvVar(env_vars::COOKIDOO_CLIENT_ID.to_string()))?;

        let client_secret = env::var(env_vars::COOKIDOO_CLIENT_SECRET).map_err(|_| {
            ConfigError::MissingEnvVar(env_vars::COOKIDOO_CLIENT_SECRET.to_string())
        })?;

        Ok(Self {
            cookidoo_credentials: CookidooCredentials::new(email, password),
            cookidoo_client_id: client_id,
            cookidoo_client_secret: client_secret,
        })
    }

    /// Returns the Cookidoo credentials.
    pub fn cookidoo_credentials(&self) -> &CookidooCredentials {
        &self.cookidoo_credentials
    }

    /// Returns the Cookidoo OAuth client ID.
    pub fn cookidoo_client_id(&self) -> &str {
        &self.cookidoo_client_id
    }

    /// Returns the Cookidoo OAuth client secret.
    pub fn cookidoo_client_secret(&self) -> &str {
        &self.cookidoo_client_secret
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
                ("COOKIDOO_CLIENT_ID", "my-client-id"),
                ("COOKIDOO_CLIENT_SECRET", "my-client-secret"),
            ],
            || {
                let config = AppConfig::from_env().unwrap();
                assert_eq!(config.cookidoo_credentials().email(), "test@example.com");
                assert_eq!(config.cookidoo_credentials().password(), "secret123");
                assert_eq!(config.cookidoo_client_id(), "my-client-id");
                assert_eq!(config.cookidoo_client_secret(), "my-client-secret");
            },
        );
    }

    #[test]
    fn returns_error_when_email_missing() {
        with_env_vars(
            &[
                ("COOKIDOO_PASSWORD", "secret123"),
                ("COOKIDOO_CLIENT_ID", "my-client-id"),
                ("COOKIDOO_CLIENT_SECRET", "my-client-secret"),
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
                ("COOKIDOO_CLIENT_ID", "my-client-id"),
                ("COOKIDOO_CLIENT_SECRET", "my-client-secret"),
            ],
            || {
                let result = AppConfig::from_env();
                assert!(matches!(result, Err(ConfigError::MissingEnvVar(_))));
            },
        );
    }

    #[test]
    fn returns_error_when_client_id_missing() {
        with_env_vars(
            &[
                ("COOKIDOO_EMAIL", "test@example.com"),
                ("COOKIDOO_PASSWORD", "secret123"),
                ("COOKIDOO_CLIENT_SECRET", "my-client-secret"),
            ],
            || {
                let result = AppConfig::from_env();
                assert!(matches!(result, Err(ConfigError::MissingEnvVar(_))));
            },
        );
    }

    #[test]
    fn returns_error_when_client_secret_missing() {
        with_env_vars(
            &[
                ("COOKIDOO_EMAIL", "test@example.com"),
                ("COOKIDOO_PASSWORD", "secret123"),
                ("COOKIDOO_CLIENT_ID", "my-client-id"),
            ],
            || {
                let result = AppConfig::from_env();
                assert!(matches!(result, Err(ConfigError::MissingEnvVar(_))));
            },
        );
    }
}
