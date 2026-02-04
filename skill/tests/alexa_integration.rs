//! Integration tests for the Alexa adapter.

use std::sync::Arc;

use async_trait::async_trait;

use alexa_cookidoo_skill::adapters::alexa::{AlexaRequest, AlexaSkillHandler};
use alexa_cookidoo_skill::domain::models::{DomainError, ShoppingListItem};
use alexa_cookidoo_skill::domain::ports::ShoppingListRepository;
use alexa_cookidoo_skill::domain::services::AddItemService;

/// Mock repository that always succeeds.
struct SuccessRepository;

#[async_trait]
impl ShoppingListRepository for SuccessRepository {
    async fn add_item(&self, _item: &ShoppingListItem) -> Result<(), DomainError> {
        Ok(())
    }
}

/// Mock repository that always fails with a repository error.
struct FailingRepository;

#[async_trait]
impl ShoppingListRepository for FailingRepository {
    async fn add_item(&self, _item: &ShoppingListItem) -> Result<(), DomainError> {
        Err(DomainError::RepositoryError(
            "Connection failed".to_string(),
        ))
    }
}

/// Mock repository that fails with an auth error.
struct AuthFailingRepository;

#[async_trait]
impl ShoppingListRepository for AuthFailingRepository {
    async fn add_item(&self, _item: &ShoppingListItem) -> Result<(), DomainError> {
        Err(DomainError::AuthenticationFailed(
            "Invalid token".to_string(),
        ))
    }
}

fn create_handler<R: ShoppingListRepository>(repo: R) -> AlexaSkillHandler<R> {
    let service = Arc::new(AddItemService::new(Arc::new(repo)));
    AlexaSkillHandler::new(service)
}

fn load_fixture(name: &str) -> AlexaRequest {
    let path = format!("tests/fixtures/{}", name);
    let content = std::fs::read_to_string(&path)
        .unwrap_or_else(|_| panic!("Failed to read fixture: {}", path));
    serde_json::from_str(&content)
        .unwrap_or_else(|e| panic!("Failed to parse fixture {}: {}", path, e))
}

#[tokio::test]
async fn launch_request_returns_welcome_message() {
    let handler = create_handler(SuccessRepository);
    let request = load_fixture("launch_request.json");

    let response = handler.handle(request).await;

    assert!(!response.response.should_end_session);
    assert!(response.response.output_speech.text.contains("Willkommen"));
}

#[tokio::test]
async fn help_request_returns_help_message() {
    let handler = create_handler(SuccessRepository);
    let request = load_fixture("help_request.json");

    let response = handler.handle(request).await;

    assert!(!response.response.should_end_session);
    assert!(response.response.output_speech.text.contains("hinzufügen"));
}

#[tokio::test]
async fn stop_request_returns_goodbye() {
    let handler = create_handler(SuccessRepository);
    let request = load_fixture("stop_request.json");

    let response = handler.handle(request).await;

    assert!(response.response.should_end_session);
    assert!(response.response.output_speech.text.contains("Wiedersehen"));
}

#[tokio::test]
async fn add_item_success_returns_confirmation() {
    let handler = create_handler(SuccessRepository);
    let request = load_fixture("add_item_request.json");

    let response = handler.handle(request).await;

    assert!(response.response.should_end_session);
    assert!(response.response.output_speech.text.contains("Testmilch"));
    assert!(response.response.output_speech.text.contains("hinzugefügt"));
}

#[tokio::test]
async fn add_item_empty_slot_returns_unknown() {
    let handler = create_handler(SuccessRepository);
    let request = load_fixture("add_item_empty_slot_request.json");

    let response = handler.handle(request).await;

    // Empty slot should be treated as unknown intent
    assert!(!response.response.should_end_session);
    assert!(response
        .response
        .output_speech
        .text
        .contains("nicht verstanden"));
}

#[tokio::test]
async fn add_item_repository_error_returns_error_message() {
    let handler = create_handler(FailingRepository);
    let request = load_fixture("add_item_request.json");

    let response = handler.handle(request).await;

    assert!(response.response.should_end_session);
    assert!(response
        .response
        .output_speech
        .text
        .contains("nicht hinzugefügt"));
}

#[tokio::test]
async fn add_item_auth_error_returns_auth_message() {
    let handler = create_handler(AuthFailingRepository);
    let request = load_fixture("add_item_request.json");

    let response = handler.handle(request).await;

    assert!(response.response.should_end_session);
    assert!(response.response.output_speech.text.contains("Anmeldung"));
}

#[tokio::test]
async fn response_version_is_correct() {
    let handler = create_handler(SuccessRepository);
    let request = load_fixture("launch_request.json");

    let response = handler.handle(request).await;

    assert_eq!(response.version, "1.0");
}

#[tokio::test]
async fn response_serializes_to_valid_json() {
    let handler = create_handler(SuccessRepository);
    let request = load_fixture("add_item_request.json");

    let response = handler.handle(request).await;

    let json = serde_json::to_string(&response);
    assert!(json.is_ok());

    let json_str = json.unwrap();
    assert!(json_str.contains("outputSpeech"));
    assert!(json_str.contains("shouldEndSession"));
}
