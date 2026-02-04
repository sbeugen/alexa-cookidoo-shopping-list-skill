use std::sync::Arc;

use tracing::info;

use crate::domain::ports::ShoppingListRepository;
use crate::domain::services::AddItemService;

use super::intent_parser::{self, ParsedIntent};
use super::models::{AlexaRequest, AlexaResponse};
use super::response_builder::ResponseBuilder;

/// Main Alexa skill handler.
pub struct AlexaSkillHandler<R: ShoppingListRepository> {
    add_item_service: Arc<AddItemService<R>>,
}

impl<R: ShoppingListRepository> AlexaSkillHandler<R> {
    /// Creates a new AlexaSkillHandler with the given service.
    pub fn new(add_item_service: Arc<AddItemService<R>>) -> Self {
        Self { add_item_service }
    }

    /// Handles an Alexa request and returns an appropriate response.
    pub async fn handle(&self, request: AlexaRequest) -> AlexaResponse {
        let intent = intent_parser::parse(&request);

        info!(intent = ?intent, "Processing Alexa request");

        match intent {
            ParsedIntent::Launch => {
                info!("Handling launch request");
                ResponseBuilder::launch()
            }

            ParsedIntent::AddItem { item_name } => {
                info!(item_name = %item_name, "Handling add item request");
                match self.add_item_service.execute(&item_name).await {
                    Ok(message) => ResponseBuilder::success(message),
                    Err(message) => ResponseBuilder::error(message),
                }
            }

            ParsedIntent::Help => {
                info!("Handling help request");
                ResponseBuilder::help()
            }

            ParsedIntent::Cancel | ParsedIntent::Stop => {
                info!("Handling cancel/stop request");
                ResponseBuilder::goodbye()
            }

            ParsedIntent::Unknown => {
                info!("Handling unknown request");
                ResponseBuilder::unknown()
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::domain::models::{DomainError, ShoppingListItem};
    use async_trait::async_trait;

    struct MockRepository {
        should_fail: bool,
    }

    impl MockRepository {
        fn new() -> Self {
            Self { should_fail: false }
        }

        fn failing() -> Self {
            Self { should_fail: true }
        }
    }

    #[async_trait]
    impl ShoppingListRepository for MockRepository {
        async fn add_item(&self, _item: &ShoppingListItem) -> Result<(), DomainError> {
            if self.should_fail {
                Err(DomainError::RepositoryError("Test error".to_string()))
            } else {
                Ok(())
            }
        }
    }

    fn make_handler(repo: MockRepository) -> AlexaSkillHandler<MockRepository> {
        let service = Arc::new(AddItemService::new(Arc::new(repo)));
        AlexaSkillHandler::new(service)
    }

    fn make_launch_request() -> AlexaRequest {
        serde_json::from_str(
            r#"{
                "version": "1.0",
                "request": {
                    "type": "LaunchRequest",
                    "requestId": "req-123",
                    "timestamp": "2024-01-27T10:00:00Z",
                    "locale": "de-DE"
                }
            }"#,
        )
        .unwrap()
    }

    fn make_add_item_request(item: &str) -> AlexaRequest {
        let json = format!(
            r#"{{
                "version": "1.0",
                "request": {{
                    "type": "IntentRequest",
                    "requestId": "req-123",
                    "timestamp": "2024-01-27T10:00:00Z",
                    "locale": "de-DE",
                    "intent": {{
                        "name": "AddItemIntent",
                        "slots": {{
                            "Item": {{"name": "Item", "value": "{item}"}}
                        }}
                    }}
                }}
            }}"#
        );
        serde_json::from_str(&json).unwrap()
    }

    fn make_help_request() -> AlexaRequest {
        serde_json::from_str(
            r#"{
                "version": "1.0",
                "request": {
                    "type": "IntentRequest",
                    "requestId": "req-123",
                    "timestamp": "2024-01-27T10:00:00Z",
                    "locale": "de-DE",
                    "intent": {"name": "AMAZON.HelpIntent", "slots": {}}
                }
            }"#,
        )
        .unwrap()
    }

    fn make_stop_request() -> AlexaRequest {
        serde_json::from_str(
            r#"{
                "version": "1.0",
                "request": {
                    "type": "IntentRequest",
                    "requestId": "req-123",
                    "timestamp": "2024-01-27T10:00:00Z",
                    "locale": "de-DE",
                    "intent": {"name": "AMAZON.StopIntent", "slots": {}}
                }
            }"#,
        )
        .unwrap()
    }

    #[tokio::test]
    async fn handles_launch_request() {
        let handler = make_handler(MockRepository::new());
        let response = handler.handle(make_launch_request()).await;

        assert!(!response.response.should_end_session);
        assert!(response.response.output_speech.text.contains("Willkommen"));
    }

    #[tokio::test]
    async fn handles_add_item_success() {
        let handler = make_handler(MockRepository::new());
        let response = handler.handle(make_add_item_request("Milch")).await;

        assert!(response.response.should_end_session);
        assert!(response.response.output_speech.text.contains("Milch"));
        assert!(response.response.output_speech.text.contains("hinzugefügt"));
    }

    #[tokio::test]
    async fn handles_add_item_failure() {
        let handler = make_handler(MockRepository::failing());
        let response = handler.handle(make_add_item_request("Milch")).await;

        assert!(response.response.should_end_session);
        assert!(response
            .response
            .output_speech
            .text
            .contains("nicht hinzugefügt"));
    }

    #[tokio::test]
    async fn handles_help_request() {
        let handler = make_handler(MockRepository::new());
        let response = handler.handle(make_help_request()).await;

        assert!(!response.response.should_end_session);
    }

    #[tokio::test]
    async fn handles_stop_request() {
        let handler = make_handler(MockRepository::new());
        let response = handler.handle(make_stop_request()).await;

        assert!(response.response.should_end_session);
        assert!(response.response.output_speech.text.contains("Wiedersehen"));
    }
}
