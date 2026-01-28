use super::models::{AlexaRequest, Request};

/// Parsed intent from an Alexa request.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ParsedIntent {
    /// User wants to add an item to the shopping list.
    AddItem { item_name: String },
    /// User requested help.
    Help,
    /// User wants to cancel.
    Cancel,
    /// User wants to stop.
    Stop,
    /// User launched the skill without a specific intent.
    Launch,
    /// Unknown or unsupported intent.
    Unknown,
}

/// Intent names from Alexa.
mod intent_names {
    pub const ADD_ITEM: &str = "AddItemIntent";
    pub const HELP: &str = "AMAZON.HelpIntent";
    pub const CANCEL: &str = "AMAZON.CancelIntent";
    pub const STOP: &str = "AMAZON.StopIntent";
    pub const FALLBACK: &str = "AMAZON.FallbackIntent";
}

/// Slot names for intents.
mod slot_names {
    pub const ITEM: &str = "Item";
}

/// Parses an Alexa request into a domain-friendly intent.
pub fn parse(request: &AlexaRequest) -> ParsedIntent {
    match &request.request {
        Request::LaunchRequest(_) => ParsedIntent::Launch,

        Request::IntentRequest(intent_req) => {
            let intent_name = intent_req.intent.name.as_str();

            match intent_name {
                intent_names::ADD_ITEM => {
                    let item_name = intent_req
                        .intent
                        .slots
                        .get(slot_names::ITEM)
                        .and_then(|slot| slot.value.clone())
                        .unwrap_or_default();

                    if item_name.is_empty() {
                        ParsedIntent::Unknown
                    } else {
                        ParsedIntent::AddItem { item_name }
                    }
                }
                intent_names::HELP => ParsedIntent::Help,
                intent_names::CANCEL => ParsedIntent::Cancel,
                intent_names::STOP => ParsedIntent::Stop,
                intent_names::FALLBACK => ParsedIntent::Unknown,
                _ => ParsedIntent::Unknown,
            }
        }

        Request::SessionEndedRequest(_) => ParsedIntent::Stop,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

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

    fn make_intent_request(intent_name: &str, slots_json: &str) -> AlexaRequest {
        let json = format!(
            r#"{{
                "version": "1.0",
                "request": {{
                    "type": "IntentRequest",
                    "requestId": "req-123",
                    "timestamp": "2024-01-27T10:00:00Z",
                    "locale": "de-DE",
                    "intent": {{
                        "name": "{intent_name}",
                        "slots": {slots_json}
                    }}
                }}
            }}"#
        );
        serde_json::from_str(&json).unwrap()
    }

    #[test]
    fn parses_launch_request() {
        let request = make_launch_request();
        assert_eq!(parse(&request), ParsedIntent::Launch);
    }

    #[test]
    fn parses_add_item_intent_with_slot() {
        let request = make_intent_request(
            "AddItemIntent",
            r#"{"Item": {"name": "Item", "value": "Milch"}}"#,
        );
        assert_eq!(
            parse(&request),
            ParsedIntent::AddItem {
                item_name: "Milch".to_string()
            }
        );
    }

    #[test]
    fn parses_add_item_intent_without_slot_as_unknown() {
        let request = make_intent_request("AddItemIntent", "{}");
        assert_eq!(parse(&request), ParsedIntent::Unknown);
    }

    #[test]
    fn parses_add_item_intent_with_empty_slot_as_unknown() {
        let request = make_intent_request(
            "AddItemIntent",
            r#"{"Item": {"name": "Item", "value": ""}}"#,
        );
        assert_eq!(parse(&request), ParsedIntent::Unknown);
    }

    #[test]
    fn parses_help_intent() {
        let request = make_intent_request("AMAZON.HelpIntent", "{}");
        assert_eq!(parse(&request), ParsedIntent::Help);
    }

    #[test]
    fn parses_cancel_intent() {
        let request = make_intent_request("AMAZON.CancelIntent", "{}");
        assert_eq!(parse(&request), ParsedIntent::Cancel);
    }

    #[test]
    fn parses_stop_intent() {
        let request = make_intent_request("AMAZON.StopIntent", "{}");
        assert_eq!(parse(&request), ParsedIntent::Stop);
    }

    #[test]
    fn parses_fallback_intent_as_unknown() {
        let request = make_intent_request("AMAZON.FallbackIntent", "{}");
        assert_eq!(parse(&request), ParsedIntent::Unknown);
    }

    #[test]
    fn parses_unknown_intent() {
        let request = make_intent_request("SomeRandomIntent", "{}");
        assert_eq!(parse(&request), ParsedIntent::Unknown);
    }
}