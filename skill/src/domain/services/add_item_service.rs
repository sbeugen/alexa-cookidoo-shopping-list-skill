use std::sync::Arc;

use tracing::{error, info};

use crate::domain::models::{DomainError, ShoppingListItem};
use crate::domain::ports::ShoppingListRepository;

/// Service for adding items to the shopping list.
///
/// This is the core use case that orchestrates the validation
/// and persistence of shopping list items.
pub struct AddItemService<R: ShoppingListRepository> {
    repository: Arc<R>,
}

impl<R: ShoppingListRepository> AddItemService<R> {
    /// Creates a new AddItemService with the given repository.
    pub fn new(repository: Arc<R>) -> Self {
        Self { repository }
    }

    /// Adds an item to the shopping list.
    ///
    /// # Arguments
    /// * `item_name` - The raw item name from user input
    ///
    /// # Returns
    /// A user-friendly message indicating success or failure.
    pub async fn execute(&self, item_name: &str) -> Result<String, String> {
        let item = match ShoppingListItem::new(item_name) {
            Ok(item) => item,
            Err(DomainError::InvalidItemName(msg)) => {
                error!(error = %msg, "Invalid item name provided");
                return Err(format!("Der Artikelname ist ungültig: {}", msg));
            }
            Err(e) => {
                error!(error = %e, "Unexpected error creating item");
                return Err("Ein unerwarteter Fehler ist aufgetreten.".to_string());
            }
        };

        match self.repository.add_item(&item).await {
            Ok(()) => {
                info!(item_name = %item.name(), "Item added to shopping list");
                Ok(format!(
                    "{} wurde zur Einkaufsliste hinzugefügt.",
                    item.name()
                ))
            }
            Err(DomainError::AuthenticationFailed(msg)) => {
                error!(error = %msg, "Authentication failed while adding item");
                Err("Die Anmeldung bei Cookidoo ist fehlgeschlagen. Bitte überprüfe deine Zugangsdaten.".to_string())
            }
            Err(DomainError::RepositoryError(msg)) => {
                error!(error = %msg, "Repository error while adding item");
                Err(
                    "Der Artikel konnte nicht hinzugefügt werden. Bitte versuche es später erneut."
                        .to_string(),
                )
            }
            Err(e) => {
                error!(error = %e, "Unexpected error adding item");
                Err("Ein unerwarteter Fehler ist aufgetreten.".to_string())
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use async_trait::async_trait;
    use std::sync::atomic::{AtomicBool, Ordering};

    struct MockRepository {
        should_fail: AtomicBool,
        fail_with_auth: AtomicBool,
    }

    impl MockRepository {
        fn new() -> Self {
            Self {
                should_fail: AtomicBool::new(false),
                fail_with_auth: AtomicBool::new(false),
            }
        }

        fn failing() -> Self {
            Self {
                should_fail: AtomicBool::new(true),
                fail_with_auth: AtomicBool::new(false),
            }
        }

        fn failing_auth() -> Self {
            Self {
                should_fail: AtomicBool::new(true),
                fail_with_auth: AtomicBool::new(true),
            }
        }
    }

    #[async_trait]
    impl ShoppingListRepository for MockRepository {
        async fn add_item(&self, _item: &ShoppingListItem) -> Result<(), DomainError> {
            if self.should_fail.load(Ordering::SeqCst) {
                if self.fail_with_auth.load(Ordering::SeqCst) {
                    Err(DomainError::AuthenticationFailed(
                        "Invalid token".to_string(),
                    ))
                } else {
                    Err(DomainError::RepositoryError(
                        "Connection failed".to_string(),
                    ))
                }
            } else {
                Ok(())
            }
        }
    }

    #[tokio::test]
    async fn execute_adds_valid_item() {
        let repo = Arc::new(MockRepository::new());
        let service = AddItemService::new(repo);

        let result = service.execute("Milk").await;

        assert!(result.is_ok());
        assert!(result.unwrap().contains("Milk"));
    }

    #[tokio::test]
    async fn execute_returns_error_for_empty_item() {
        let repo = Arc::new(MockRepository::new());
        let service = AddItemService::new(repo);

        let result = service.execute("").await;

        assert!(result.is_err());
    }

    #[tokio::test]
    async fn execute_returns_error_on_repository_failure() {
        let repo = Arc::new(MockRepository::failing());
        let service = AddItemService::new(repo);

        let result = service.execute("Milk").await;

        assert!(result.is_err());
        assert!(result.unwrap_err().contains("nicht hinzugefügt"));
    }

    #[tokio::test]
    async fn execute_returns_auth_error_message() {
        let repo = Arc::new(MockRepository::failing_auth());
        let service = AddItemService::new(repo);

        let result = service.execute("Milk").await;

        assert!(result.is_err());
        assert!(result.unwrap_err().contains("Anmeldung"));
    }
}
