use std::collections::HashMap;

use serde::{Deserialize, Serialize};

// ============================================================================
// Request Models
// ============================================================================

/// Top-level Alexa request wrapper.
#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AlexaRequest {
    pub version: String,
    pub session: Option<Session>,
    pub request: Request,
}

/// Session information from Alexa.
#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Session {
    pub new: bool,
    pub session_id: String,
    pub application: Application,
    pub user: User,
}

/// Skill application information.
#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Application {
    pub application_id: String,
}

/// User information.
#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct User {
    pub user_id: String,
}

/// Alexa request types.
#[derive(Debug, Deserialize)]
#[serde(tag = "type")]
pub enum Request {
    LaunchRequest(LaunchRequest),
    IntentRequest(IntentRequest),
    SessionEndedRequest(SessionEndedRequest),
}

/// Launch request when user opens the skill.
#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LaunchRequest {
    pub request_id: String,
    pub timestamp: String,
    pub locale: String,
}

/// Intent request when user speaks a command.
#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct IntentRequest {
    pub request_id: String,
    pub timestamp: String,
    pub locale: String,
    pub intent: Intent,
}

/// Session ended request.
#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SessionEndedRequest {
    pub request_id: String,
    pub timestamp: String,
    pub locale: String,
    pub reason: String,
}

/// Intent with name and slots.
#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Intent {
    pub name: String,
    #[serde(default)]
    pub slots: HashMap<String, Slot>,
}

/// Slot value from user speech.
#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Slot {
    pub name: String,
    pub value: Option<String>,
}

// ============================================================================
// Response Models
// ============================================================================

/// Top-level Alexa response.
#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct AlexaResponse {
    pub version: String,
    pub response: ResponseBody,
}

/// Response body containing speech and session control.
#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ResponseBody {
    pub output_speech: OutputSpeech,
    pub should_end_session: bool,
}

/// Output speech in plain text format.
#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct OutputSpeech {
    #[serde(rename = "type")]
    pub speech_type: String,
    pub text: String,
}

impl OutputSpeech {
    /// Creates a plain text output speech.
    pub fn plain_text(text: impl Into<String>) -> Self {
        Self {
            speech_type: "PlainText".to_string(),
            text: text.into(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn deserializes_launch_request() {
        let json = r#"{
            "version": "1.0",
            "session": {
                "new": true,
                "sessionId": "session-123",
                "application": {"applicationId": "app-123"},
                "user": {"userId": "user-123"}
            },
            "request": {
                "type": "LaunchRequest",
                "requestId": "req-123",
                "timestamp": "2024-01-27T10:00:00Z",
                "locale": "de-DE"
            }
        }"#;

        let request: AlexaRequest = serde_json::from_str(json).unwrap();
        assert_eq!(request.version, "1.0");
        assert!(matches!(request.request, Request::LaunchRequest(_)));
    }

    #[test]
    fn deserializes_intent_request_with_slot() {
        let json = r#"{
            "version": "1.0",
            "session": {
                "new": false,
                "sessionId": "session-123",
                "application": {"applicationId": "app-123"},
                "user": {"userId": "user-123"}
            },
            "request": {
                "type": "IntentRequest",
                "requestId": "req-123",
                "timestamp": "2024-01-27T10:00:00Z",
                "locale": "de-DE",
                "intent": {
                    "name": "AddItemIntent",
                    "slots": {
                        "Item": {
                            "name": "Item",
                            "value": "Milch"
                        }
                    }
                }
            }
        }"#;

        let request: AlexaRequest = serde_json::from_str(json).unwrap();
        if let Request::IntentRequest(intent_req) = &request.request {
            assert_eq!(intent_req.intent.name, "AddItemIntent");
            assert_eq!(
                intent_req.intent.slots.get("Item").unwrap().value,
                Some("Milch".to_string())
            );
        } else {
            panic!("Expected IntentRequest");
        }
    }

    #[test]
    fn deserializes_intent_request_without_slots() {
        let json = r#"{
            "version": "1.0",
            "request": {
                "type": "IntentRequest",
                "requestId": "req-123",
                "timestamp": "2024-01-27T10:00:00Z",
                "locale": "de-DE",
                "intent": {
                    "name": "AMAZON.HelpIntent"
                }
            }
        }"#;

        let request: AlexaRequest = serde_json::from_str(json).unwrap();
        if let Request::IntentRequest(intent_req) = &request.request {
            assert_eq!(intent_req.intent.name, "AMAZON.HelpIntent");
            assert!(intent_req.intent.slots.is_empty());
        } else {
            panic!("Expected IntentRequest");
        }
    }

    #[test]
    fn serializes_response() {
        let response = AlexaResponse {
            version: "1.0".to_string(),
            response: ResponseBody {
                output_speech: OutputSpeech::plain_text("Hello"),
                should_end_session: true,
            },
        };

        let json = serde_json::to_string(&response).unwrap();
        assert!(json.contains("\"version\":\"1.0\""));
        assert!(json.contains("\"text\":\"Hello\""));
        assert!(json.contains("\"shouldEndSession\":true"));
    }
}