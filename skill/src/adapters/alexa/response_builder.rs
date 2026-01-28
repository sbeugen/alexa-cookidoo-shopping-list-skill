use super::models::{AlexaResponse, OutputSpeech, ResponseBody};

/// German response messages.
mod messages {
    pub const WELCOME: &str = "Willkommen bei der Cookidoo Einkaufsliste. \
        Du kannst Artikel hinzufügen, indem du zum Beispiel sagst: \
        Füge Milch hinzu.";

    pub const HELP: &str = "Du kannst Artikel zu deiner Cookidoo Einkaufsliste hinzufügen. \
        Sage zum Beispiel: Füge Milch hinzu, oder: Ich brauche Eier. \
        Was möchtest du hinzufügen?";

    pub const GOODBYE: &str = "Auf Wiedersehen!";

    pub const UNKNOWN: &str = "Das habe ich leider nicht verstanden. \
        Bitte sage zum Beispiel: Füge Milch hinzu.";
}

/// Builder for Alexa responses.
pub struct ResponseBuilder;

impl ResponseBuilder {
    /// Creates a success response with the given message, ending the session.
    pub fn success(message: impl Into<String>) -> AlexaResponse {
        Self::build(message, true)
    }

    /// Creates an error response with the given message, ending the session.
    pub fn error(message: impl Into<String>) -> AlexaResponse {
        Self::build(message, true)
    }

    /// Creates a welcome message response, keeping the session open.
    pub fn launch() -> AlexaResponse {
        Self::build(messages::WELCOME, false)
    }

    /// Creates a help response, keeping the session open.
    pub fn help() -> AlexaResponse {
        Self::build(messages::HELP, false)
    }

    /// Creates a goodbye response, ending the session.
    pub fn goodbye() -> AlexaResponse {
        Self::build(messages::GOODBYE, true)
    }

    /// Creates an unknown intent response, keeping the session open.
    pub fn unknown() -> AlexaResponse {
        Self::build(messages::UNKNOWN, false)
    }

    fn build(text: impl Into<String>, end_session: bool) -> AlexaResponse {
        AlexaResponse {
            version: "1.0".to_string(),
            response: ResponseBody {
                output_speech: OutputSpeech::plain_text(text),
                should_end_session: end_session,
            },
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn success_ends_session() {
        let response = ResponseBuilder::success("Item added");
        assert!(response.response.should_end_session);
        assert_eq!(response.response.output_speech.text, "Item added");
    }

    #[test]
    fn error_ends_session() {
        let response = ResponseBuilder::error("Something went wrong");
        assert!(response.response.should_end_session);
    }

    #[test]
    fn launch_keeps_session_open() {
        let response = ResponseBuilder::launch();
        assert!(!response.response.should_end_session);
        assert!(response.response.output_speech.text.contains("Willkommen"));
    }

    #[test]
    fn help_keeps_session_open() {
        let response = ResponseBuilder::help();
        assert!(!response.response.should_end_session);
    }

    #[test]
    fn goodbye_ends_session() {
        let response = ResponseBuilder::goodbye();
        assert!(response.response.should_end_session);
        assert!(response.response.output_speech.text.contains("Wiedersehen"));
    }

    #[test]
    fn unknown_keeps_session_open() {
        let response = ResponseBuilder::unknown();
        assert!(!response.response.should_end_session);
    }

    #[test]
    fn response_version_is_1_0() {
        let response = ResponseBuilder::success("Test");
        assert_eq!(response.version, "1.0");
    }
}