use serde::{Deserialize, Serialize};

#[derive(Serialize)]
pub struct ChatMessage {
    pub role: String,
    pub content: String,
}

#[derive(Serialize)]
pub struct DeepSeekRequest {
    pub model: String,
    pub messages: Vec<ChatMessage>,
    pub stream: bool,
}

#[derive(Deserialize, Debug, Default)]
pub struct DeepSeekResponse {
    #[serde(default)]
    pub choices: Vec<Choice>,
    #[serde(default)]
    pub error: Option<ApiError>,
}

#[derive(Deserialize, Debug, Default)]
pub struct Choice {
    pub message: ChoiceMessage,
}

#[derive(Deserialize, Debug, Default)]
pub struct ChoiceMessage {
    pub content: String,
    #[serde(default)]
    pub role: Option<String>,
}

#[derive(Deserialize, Debug, Default)]
pub struct ApiError {
    pub message: String,
    #[serde(default)]
    pub code: Option<String>,
    #[serde(default)]
    pub r#type: Option<String>,
}
