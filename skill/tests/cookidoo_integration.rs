//! Integration tests for the Cookidoo adapter using wiremock.

use std::sync::Arc;
use std::time::Duration;

use wiremock::matchers::{body_string_contains, header, method, path};
use wiremock::{Mock, MockServer, ResponseTemplate};

use alexa_cookidoo_skill::adapters::cookidoo::{
    CookidooAuthAdapter, CookidooClient, CookidooShoppingListAdapter,
};
use alexa_cookidoo_skill::domain::models::{CookidooCredentials, ShoppingListItem};
use alexa_cookidoo_skill::domain::ports::ShoppingListRepository;

fn test_credentials() -> CookidooCredentials {
    CookidooCredentials::new("test@example.com", "testpassword")
}

fn test_auth_header() -> String {
    "Basic a3VwZmVyd2Vyay1jbGllbnQtbndvdDpMczUwT04xd295U3FzMWRDZEpnZQ==".to_string()
}

fn auth_success_response() -> ResponseTemplate {
    ResponseTemplate::new(200).set_body_json(serde_json::json!({
        "access_token": "test-access-token",
        "refresh_token": "test-refresh-token",
        "expires_in": 3600,
        "token_type": "Bearer"
    }))
}

fn add_item_success_response() -> ResponseTemplate {
    ResponseTemplate::new(200).set_body_json(serde_json::json!({
        "data": [{
            "id": "test-item-id",
            "name": "Milk",
            "isOwned": false
        }]
    }))
}

#[tokio::test]
async fn authentication_success() {
    let mock_server = MockServer::start().await;

    Mock::given(method("POST"))
        .and(path("/ciam/auth/token"))
        .and(header("Authorization", "Basic a3VwZmVyd2Vyay1jbGllbnQtbndvdDpMczUwT04xd295U3FzMWRDZEpnZQ=="))
        .and(body_string_contains("grant_type=password"))
        .respond_with(auth_success_response())
        .mount(&mock_server)
        .await;

    let client = CookidooClient::with_base_url(mock_server.uri());
    let auth = CookidooAuthAdapter::new(client, test_credentials(), test_auth_header());

    let token = auth.get_valid_token().await;

    assert!(token.is_ok());
    assert_eq!(token.unwrap(), "test-access-token");
}

#[tokio::test]
async fn authentication_failure_invalid_credentials() {
    let mock_server = MockServer::start().await;

    Mock::given(method("POST"))
        .and(path("/ciam/auth/token"))
        .respond_with(ResponseTemplate::new(401).set_body_json(serde_json::json!({
            "error": "invalid_grant",
            "error_description": "Invalid credentials"
        })))
        .mount(&mock_server)
        .await;

    let client = CookidooClient::with_base_url(mock_server.uri());
    let auth = CookidooAuthAdapter::new(client, test_credentials(), test_auth_header());

    let token = auth.get_valid_token().await;

    assert!(token.is_err());
}

#[tokio::test]
async fn token_caching_reuses_valid_token() {
    let mock_server = MockServer::start().await;

    Mock::given(method("POST"))
        .and(path("/ciam/auth/token"))
        .respond_with(auth_success_response())
        .expect(1) // Should only be called once
        .mount(&mock_server)
        .await;

    let client = CookidooClient::with_base_url(mock_server.uri());
    let auth = CookidooAuthAdapter::new(client, test_credentials(), test_auth_header());

    // First call - should authenticate
    let token1 = auth.get_valid_token().await.unwrap();

    // Second call - should use cache
    let token2 = auth.get_valid_token().await.unwrap();

    assert_eq!(token1, token2);
}

#[tokio::test]
async fn add_item_success() {
    let mock_server = MockServer::start().await;

    // Mock auth endpoint
    Mock::given(method("POST"))
        .and(path("/ciam/auth/token"))
        .respond_with(auth_success_response())
        .mount(&mock_server)
        .await;

    // Mock add item endpoint
    Mock::given(method("POST"))
        .and(path("/shopping/de-DE/additional-items/add"))
        .and(header("Authorization", "Bearer test-access-token"))
        .and(body_string_contains("itemsValue"))
        .respond_with(add_item_success_response())
        .mount(&mock_server)
        .await;

    let client = CookidooClient::with_base_url(mock_server.uri());
    let auth = Arc::new(CookidooAuthAdapter::new(client.clone(), test_credentials(), test_auth_header()));
    let shopping_list = CookidooShoppingListAdapter::new(client, auth);

    let item = ShoppingListItem::new("Milk").unwrap();
    let result = shopping_list.add_item(&item).await;

    assert!(result.is_ok());
}

#[tokio::test]
async fn add_item_retries_on_401() {
    let mock_server = MockServer::start().await;

    // Mock auth endpoint - will be called twice (initial + retry)
    Mock::given(method("POST"))
        .and(path("/ciam/auth/token"))
        .respond_with(auth_success_response())
        .expect(2)
        .mount(&mock_server)
        .await;

    // First add item call returns 401
    Mock::given(method("POST"))
        .and(path("/shopping/de-DE/additional-items/add"))
        .respond_with(ResponseTemplate::new(401))
        .up_to_n_times(1)
        .mount(&mock_server)
        .await;

    // Second add item call succeeds
    Mock::given(method("POST"))
        .and(path("/shopping/de-DE/additional-items/add"))
        .respond_with(add_item_success_response())
        .mount(&mock_server)
        .await;

    let client = CookidooClient::with_base_url(mock_server.uri());
    let auth = Arc::new(CookidooAuthAdapter::new(client.clone(), test_credentials(), test_auth_header()));
    let shopping_list = CookidooShoppingListAdapter::new(client, auth);

    let item = ShoppingListItem::new("Milk").unwrap();
    let result = shopping_list.add_item(&item).await;

    assert!(result.is_ok());
}

#[tokio::test]
async fn add_item_fails_on_server_error() {
    let mock_server = MockServer::start().await;

    Mock::given(method("POST"))
        .and(path("/ciam/auth/token"))
        .respond_with(auth_success_response())
        .mount(&mock_server)
        .await;

    Mock::given(method("POST"))
        .and(path("/shopping/de-DE/additional-items/add"))
        .respond_with(ResponseTemplate::new(500).set_body_string("Internal Server Error"))
        .mount(&mock_server)
        .await;

    let client = CookidooClient::with_base_url(mock_server.uri());
    let auth = Arc::new(CookidooAuthAdapter::new(client.clone(), test_credentials(), test_auth_header()));
    let shopping_list = CookidooShoppingListAdapter::new(client, auth);

    let item = ShoppingListItem::new("Milk").unwrap();
    let result = shopping_list.add_item(&item).await;

    assert!(result.is_err());
}

#[tokio::test]
async fn token_refresh_on_expiry() {
    let mock_server = MockServer::start().await;

    // Initial auth with short expiry
    Mock::given(method("POST"))
        .and(path("/ciam/auth/token"))
        .and(body_string_contains("grant_type=password"))
        .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!({
            "access_token": "short-lived-token",
            "refresh_token": "test-refresh-token",
            "expires_in": 1, // Expires in 1 second
            "token_type": "Bearer"
        })))
        .up_to_n_times(1)
        .mount(&mock_server)
        .await;

    // Refresh token endpoint
    Mock::given(method("POST"))
        .and(path("/ciam/auth/token"))
        .and(body_string_contains("grant_type=refresh_token"))
        .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!({
            "access_token": "refreshed-token",
            "refresh_token": "new-refresh-token",
            "expires_in": 3600,
            "token_type": "Bearer"
        })))
        .mount(&mock_server)
        .await;

    let client = CookidooClient::with_base_url(mock_server.uri());
    let auth = CookidooAuthAdapter::new(client, test_credentials(), test_auth_header());

    // First call - get initial token
    let token1 = auth.get_valid_token().await.unwrap();
    assert_eq!(token1, "short-lived-token");

    // Wait for token to need refresh (within 5-minute buffer of 1-second expiry = immediate)
    tokio::time::sleep(Duration::from_millis(100)).await;

    // Second call - should refresh
    let token2 = auth.get_valid_token().await.unwrap();
    assert_eq!(token2, "refreshed-token");
}