use std::time::Duration;

use reqwest::Client;

/// Default timeout for HTTP requests.
const DEFAULT_TIMEOUT: Duration = Duration::from_secs(30);

/// Default base URL for the Cookidoo API (Germany).
const DEFAULT_BASE_URL: &str = "https://de.tmmobile.vorwerk-digital.com";

/// HTTP client wrapper for Cookidoo API requests.
#[derive(Clone)]
pub struct CookidooClient {
    client: Client,
    base_url: String,
}

impl CookidooClient {
    /// Creates a new CookidooClient with default settings.
    pub fn new() -> Self {
        Self::with_base_url(DEFAULT_BASE_URL)
    }

    /// Creates a new CookidooClient with a custom base URL.
    pub fn with_base_url(base_url: impl Into<String>) -> Self {
        let client = Client::builder()
            .timeout(DEFAULT_TIMEOUT)
            .user_agent("AlexaCookidooSkill/1.0")
            .build()
            .expect("Failed to create HTTP client");

        Self {
            client,
            base_url: base_url.into(),
        }
    }

    /// Returns the underlying reqwest client.
    pub fn inner(&self) -> &Client {
        &self.client
    }

    /// Returns the base URL.
    pub fn base_url(&self) -> &str {
        &self.base_url
    }

    /// Builds a full URL from a path.
    pub fn url(&self, path: &str) -> String {
        format!("{}{}", self.base_url, path)
    }
}

impl Default for CookidooClient {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn creates_client_with_default_base_url() {
        let client = CookidooClient::new();
        assert_eq!(client.base_url(), DEFAULT_BASE_URL);
    }

    #[test]
    fn creates_client_with_custom_base_url() {
        let client = CookidooClient::with_base_url("https://example.com");
        assert_eq!(client.base_url(), "https://example.com");
    }

    #[test]
    fn builds_full_url() {
        let client = CookidooClient::with_base_url("https://example.com");
        assert_eq!(client.url("/api/test"), "https://example.com/api/test");
    }
}