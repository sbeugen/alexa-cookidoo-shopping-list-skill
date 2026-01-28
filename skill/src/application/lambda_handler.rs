use lambda_runtime::LambdaEvent;
use serde_json::Value;
use tracing::{error, info};

use crate::adapters::alexa::AlexaRequest;
use crate::adapters::alexa::AlexaSkillHandler;
use crate::domain::ports::ShoppingListRepository;

/// Handles an incoming Lambda event.
///
/// This function:
/// 1. Parses the incoming event as an Alexa request
/// 2. Delegates to the Alexa skill handler
/// 3. Returns the response as JSON
///
/// # Errors
/// Returns an error if the request cannot be parsed or if serialization fails.
pub async fn handle_request<R: ShoppingListRepository>(
    event: LambdaEvent<Value>,
    handler: &AlexaSkillHandler<R>,
) -> Result<Value, lambda_runtime::Error> {
    let (payload, _context) = event.into_parts();

    info!("Received Alexa request");

    // Parse the incoming request
    let alexa_request: AlexaRequest = match serde_json::from_value(payload) {
        Ok(req) => req,
        Err(e) => {
            error!(error = %e, "Failed to parse Alexa request");
            return Ok(error_response("Fehler beim Verarbeiten der Anfrage."));
        }
    };

    // Handle the request
    let response = handler.handle(alexa_request).await;

    // Serialize the response
    match serde_json::to_value(&response) {
        Ok(value) => {
            info!("Sending Alexa response");
            Ok(value)
        }
        Err(e) => {
            error!(error = %e, "Failed to serialize Alexa response");
            Ok(error_response("Interner Fehler."))
        }
    }
}

/// Creates a generic error response for Alexa.
fn error_response(message: &str) -> Value {
    serde_json::json!({
        "version": "1.0",
        "response": {
            "outputSpeech": {
                "type": "PlainText",
                "text": message
            },
            "shouldEndSession": true
        }
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use lambda_runtime::Context;
    use std::sync::Arc;
    use async_trait::async_trait;
    use crate::domain::models::{DomainError, ShoppingListItem};
    use crate::domain::ports::ShoppingListRepository;
    use crate::domain::services::AddItemService;

    struct MockRepository;

    #[async_trait]
    impl ShoppingListRepository for MockRepository {
        async fn add_item(&self, _item: &ShoppingListItem) -> Result<(), DomainError> {
            Ok(())
        }
    }

    fn make_mock_handler() -> AlexaSkillHandler<MockRepository> {
        let service = Arc::new(AddItemService::new(Arc::new(MockRepository)));
        AlexaSkillHandler::new(service)
    }

    fn make_lambda_event(payload: Value) -> LambdaEvent<Value> {
        let context = Context::default();
        LambdaEvent::new(payload, context)
    }

    #[tokio::test]
    async fn handles_valid_launch_request() {
        let handler = make_mock_handler();
        let payload = serde_json::json!({
            "version": "1.0",
            "request": {
                "type": "LaunchRequest",
                "requestId": "req-123",
                "timestamp": "2024-01-27T10:00:00Z",
                "locale": "de-DE"
            }
        });

        let event = make_lambda_event(payload);
        let result = handle_request(event, &handler).await;

        assert!(result.is_ok());
        let response = result.unwrap();
        assert_eq!(response["version"], "1.0");
        assert!(response["response"]["outputSpeech"]["text"]
            .as_str()
            .unwrap()
            .contains("Willkommen"));
    }

    #[tokio::test]
    async fn handles_invalid_json() {
        let handler = make_mock_handler();
        let payload = serde_json::json!({
            "invalid": "request"
        });

        let event = make_lambda_event(payload);
        let result = handle_request(event, &handler).await;

        assert!(result.is_ok());
        let response = result.unwrap();
        assert!(response["response"]["shouldEndSession"].as_bool().unwrap());
    }
}