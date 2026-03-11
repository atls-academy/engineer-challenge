use reqwest::StatusCode;

#[derive(Debug)]
pub struct KratosFlowError {
    pub status: StatusCode,
    pub body: serde_json::Value,
}

impl KratosFlowError {
    pub fn network(e: impl std::fmt::Display) -> Self {
        Self {
            status: StatusCode::INTERNAL_SERVER_ERROR,
            body: serde_json::json!({ "error": e.to_string() }),
        }
    }

    pub fn message_id(&self) -> u64 {
        self.body["ui"]["messages"][0]["id"].as_u64().unwrap_or(0)
    }

    pub fn message_text(&self) -> &str {
        self.body["ui"]["messages"][0]["text"]
            .as_str()
            .unwrap_or("Unknown error")
    }

    pub fn node_message_text(&self, field: &str) -> Option<&str> {
        self.body["ui"]["nodes"]
            .as_array()?
            .iter()
            .find(|n| n["attributes"]["name"].as_str() == Some(field))?["messages"][0]["text"]
            .as_str()
    }

    pub fn is_browser_location_change_required(&self) -> bool {
        self.status == StatusCode::UNPROCESSABLE_ENTITY
            && self.body["error"]["id"].as_str() == Some("browser_location_change_required")
    }

    pub fn redirect_browser_to(&self) -> Option<&str> {
        self.body["redirect_browser_to"].as_str()
    }
}

impl std::fmt::Display for KratosFlowError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Kratos error (status {}): {}", self.status, self.body)
    }
}

impl std::error::Error for KratosFlowError {}
