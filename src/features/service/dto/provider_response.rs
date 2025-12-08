use serde::Serialize;

#[derive(Debug, Clone, Serialize)]
pub struct ProviderResponse {
    pub message: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub usage: Option<UsageMetrics>,
}

impl ProviderResponse {
    pub fn new(message: String, usage: Option<UsageMetrics>) -> Self {
        Self { message, usage }
    }
}

#[derive(Debug, Clone, Serialize)]
pub struct UsageMetrics {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub completion_tokens: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub prompt_tokens: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub total_tokens: Option<i64>,
}

impl UsageMetrics {
    pub fn new(
        completion_tokens: Option<i64>,
        prompt_tokens: Option<i64>,
        total_tokens: Option<i64>,
    ) -> Self {
        Self {
            completion_tokens,
            prompt_tokens,
            total_tokens,
        }
    }
}
