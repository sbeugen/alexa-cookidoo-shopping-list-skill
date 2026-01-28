use std::sync::Arc;

use crate::adapters::alexa::AlexaSkillHandler;
use crate::adapters::cookidoo::{
    CookidooAuthAdapter, CookidooClient, CookidooShoppingListAdapter, TokenCache,
};
use crate::domain::services::AddItemService;

use super::config::AppConfig;

/// Dependency injection container holding all wired components.
///
/// This container is created once at Lambda cold start and reused
/// across warm invocations for optimal performance.
pub struct Container {
    handler: AlexaSkillHandler<CookidooShoppingListAdapter>,
}

impl Container {
    /// Creates a new container with all dependencies wired together.
    pub fn new(config: AppConfig) -> Self {
        // Create shared HTTP client
        let client = CookidooClient::new();

        // Create shared token cache (survives across invocations)
        let token_cache = Arc::new(TokenCache::new());

        // Create auth adapter with shared cache
        let auth_adapter = Arc::new(CookidooAuthAdapter::with_cache(
            client.clone(),
            config.cookidoo_credentials().clone(),
            config.cookidoo_auth_header().to_string(),
            token_cache,
        ));

        // Create shopping list adapter
        let shopping_list_adapter = Arc::new(CookidooShoppingListAdapter::new(
            client,
            auth_adapter,
        ));

        // Create domain service
        let add_item_service = Arc::new(AddItemService::new(shopping_list_adapter));

        // Create Alexa handler
        let handler = AlexaSkillHandler::new(add_item_service);

        Self { handler }
    }

    /// Returns a reference to the Alexa skill handler.
    pub fn handler(&self) -> &AlexaSkillHandler<CookidooShoppingListAdapter> {
        &self.handler
    }
}