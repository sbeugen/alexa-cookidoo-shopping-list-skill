use async_trait::async_trait;

use crate::domain::models::{DomainError, ShoppingListItem};

/// Port for shopping list operations.
///
/// Implementations of this trait handle the actual persistence
/// or API calls to manage shopping list items.
#[async_trait]
pub trait ShoppingListRepository: Send + Sync {
    /// Adds an item to the shopping list.
    ///
    /// # Errors
    /// Returns `DomainError::RepositoryError` if the operation fails.
    async fn add_item(&self, item: &ShoppingListItem) -> Result<(), DomainError>;
}