use std::sync::RwLock;

use crate::domain::models::AuthToken;

/// Thread-safe in-memory token cache.
///
/// This cache survives across Lambda warm invocations, allowing token reuse
/// without re-authentication on every request.
pub struct TokenCache {
    token: RwLock<Option<AuthToken>>,
}

impl TokenCache {
    /// Creates a new empty token cache.
    pub fn new() -> Self {
        Self {
            token: RwLock::new(None),
        }
    }

    /// Gets a clone of the cached token if present.
    pub fn get(&self) -> Option<AuthToken> {
        self.token.read().ok()?.clone()
    }

    /// Stores a token in the cache.
    pub fn set(&self, token: AuthToken) {
        if let Ok(mut guard) = self.token.write() {
            *guard = Some(token);
        }
    }

    /// Clears the cached token.
    pub fn clear(&self) {
        if let Ok(mut guard) = self.token.write() {
            *guard = None;
        }
    }

    /// Returns true if the cache contains a valid (non-expired) token.
    pub fn is_valid(&self) -> bool {
        self.get().map(|t| !t.is_expired()).unwrap_or(false)
    }

    /// Returns true if the cached token needs refresh.
    pub fn needs_refresh(&self) -> bool {
        self.get().map(|t| t.needs_refresh()).unwrap_or(true)
    }
}

impl Default for TokenCache {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::time::Duration;

    #[test]
    fn new_cache_is_empty() {
        let cache = TokenCache::new();
        assert!(cache.get().is_none());
        assert!(!cache.is_valid());
    }

    #[test]
    fn stores_and_retrieves_token() {
        let cache = TokenCache::new();
        let token = AuthToken::new("access", "refresh", Duration::from_secs(3600));

        cache.set(token);

        let retrieved = cache.get().unwrap();
        assert_eq!(retrieved.access_token(), "access");
    }

    #[test]
    fn clears_token() {
        let cache = TokenCache::new();
        cache.set(AuthToken::new(
            "access",
            "refresh",
            Duration::from_secs(3600),
        ));

        cache.clear();

        assert!(cache.get().is_none());
    }

    #[test]
    fn is_valid_returns_false_for_expired_token() {
        let cache = TokenCache::new();
        cache.set(AuthToken::new("access", "refresh", Duration::ZERO));

        assert!(!cache.is_valid());
    }

    #[test]
    fn is_valid_returns_true_for_fresh_token() {
        let cache = TokenCache::new();
        cache.set(AuthToken::new(
            "access",
            "refresh",
            Duration::from_secs(3600),
        ));

        assert!(cache.is_valid());
    }

    #[test]
    fn needs_refresh_returns_true_for_empty_cache() {
        let cache = TokenCache::new();
        assert!(cache.needs_refresh());
    }
}
