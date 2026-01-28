use std::sync::Arc;

use async_trait::async_trait;
use tracing::{debug, error, info};

use crate::domain::models::{DomainError, ShoppingListItem};
use crate::domain::ports::ShoppingListRepository;

use super::auth::CookidooAuthAdapter;
use super::client::CookidooClient;
use super::error::CookidooError;
use super::models::AddItemRequest;

/// Shopping list API endpoint path for additional items.
const SHOPPING_LIST_ENDPOINT: &str = "/shopping/de-DE/additional-items/add";

/// Cookidoo shopping list adapter implementing the ShoppingListRepository port.
pub struct CookidooShoppingListAdapter {
    client: CookidooClient,
    auth: Arc<CookidooAuthAdapter>,
}

impl CookidooShoppingListAdapter {
    /// Creates a new CookidooShoppingListAdapter.
    pub fn new(client: CookidooClient, auth: Arc<CookidooAuthAdapter>) -> Self {
        Self { client, auth }
    }

    async fn add_item_internal(&self, item: &ShoppingListItem) -> Result<(), CookidooError> {
        let token = self.auth.get_valid_token().await?;
        let url = self.client.url(SHOPPING_LIST_ENDPOINT);
        let request_body = AddItemRequest::new(item.name());

        debug!(item_name = %item.name(), "Adding item to shopping list");

        let response = self
            .client
            .inner()
            .post(&url)
            .header("Authorization", format!("Bearer {}", token))
            .json(&request_body)
            .send()
            .await?;

        let status = response.status();

        if status.is_success() {
            info!(item_name = %item.name(), "Item added successfully");
            Ok(())
        } else if status.as_u16() == 401 {
            // Token might have expired between get_valid_token and now
            // Clear cache and retry once
            error!("Received 401, clearing token cache");
            self.auth.cache().clear();

            let new_token = self.auth.get_valid_token().await?;
            let retry_response = self
                .client
                .inner()
                .post(&url)
                .header("Authorization", format!("Bearer {}", new_token))
                .json(&request_body)
                .send()
                .await?;

            let retry_status = retry_response.status();
            if retry_status.is_success() {
                info!(item_name = %item.name(), "Item added successfully on retry");
                Ok(())
            } else {
                let body = retry_response.text().await.unwrap_or_default();
                error!(status = %retry_status, body = %body, "Failed to add item after retry");
                Err(CookidooError::AuthenticationError(
                    "Authentication failed after retry".to_string(),
                ))
            }
        } else {
            let body = response.text().await.unwrap_or_default();
            error!(status = %status, body = %body, "Failed to add item");
            Err(CookidooError::HttpError {
                status: status.as_u16(),
                message: body,
            })
        }
    }
}

#[async_trait]
impl ShoppingListRepository for CookidooShoppingListAdapter {
    async fn add_item(&self, item: &ShoppingListItem) -> Result<(), DomainError> {
        self.add_item_internal(item).await.map_err(|e| e.into())
    }
}