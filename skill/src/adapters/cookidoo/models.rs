use serde::{Deserialize, Serialize};

/// Response from the Cookidoo OAuth token endpoint.
#[derive(Debug, Deserialize)]
pub struct CookidooAuthResponse {
    pub access_token: String,
    pub refresh_token: String,
    /// Token lifetime in seconds
    pub expires_in: u64,
}

/// Request body for adding items to the shopping list.
#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct AddItemRequest {
    pub items_value: Vec<String>,
}

impl AddItemRequest {
    pub fn new(name: impl Into<String>) -> Self {
        Self {
            items_value: vec![name.into()],
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn deserializes_auth_response() {
        let json = r#"{
            "access_token": "abc123",
            "refresh_token": "xyz789",
            "expires_in": 3600,
            "token_type": "Bearer"
        }"#;

        let response: CookidooAuthResponse = serde_json::from_str(json).unwrap();
        assert_eq!(response.access_token, "abc123");
        assert_eq!(response.refresh_token, "xyz789");
        assert_eq!(response.expires_in, 3600);
    }

    #[test]
    fn serializes_add_item_request() {
        let request = AddItemRequest::new("Milk");
        let json = serde_json::to_string(&request).unwrap();
        assert_eq!(json, r#"{"itemsValue":["Milk"]}"#);
    }
}