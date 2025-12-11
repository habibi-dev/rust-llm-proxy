use crate::features::service::constants::{
    DEFAULT_DEEPSEEK_MAX_OUTPUT_TOKENS, DEFAULT_DEEPSEEK_MODEL,
};
use crate::features::service::dto::chat_prompt::ChatPrompt;
use crate::features::service::models::deepseek::types::{
    ChatMessage, DeepSeekRequest, DeepSeekResponse,
};
use reqwest::Client;
use serde_json::from_str;

pub async fn deepseek(
    prompt: &ChatPrompt,
    api_key: &str,
    model: &str,
) -> Result<String, Box<dyn std::error::Error>> {
    let resolved_model = if model.trim().is_empty() {
        DEFAULT_DEEPSEEK_MODEL.to_string()
    } else {
        model.to_string()
    };

    let mut messages = Vec::new();

    if prompt.has_system_message() {
        messages.push(ChatMessage {
            role: "system".to_string(),
            content: prompt.system_message.clone().unwrap_or_default(),
        });
    }

    messages.push(ChatMessage {
        role: "user".to_string(),
        content: prompt.user_message.clone(),
    });

    // Enforce the provider's maximum output tokens to avoid truncated responses.
    let request = DeepSeekRequest {
        model: resolved_model,
        messages,
        max_tokens: Some(DEFAULT_DEEPSEEK_MAX_OUTPUT_TOKENS),
        stream: false,
    };

    let client = Client::new();
    let url = "https://api.deepseek.com/chat/completions";

    let response = client
        .post(url)
        .header("Content-Type", "application/json")
        .header("Authorization", format!("Bearer {api_key}"))
        .json(&request)
        .send()
        .await?;
    let response_body = response.text().await?;
    let deepseek_response: DeepSeekResponse = from_str(&response_body).map_err(|error| {
        format!("Failed to parse DeepSeek response: {error}; body: {response_body}")
    })?;

    if let Some(error) = deepseek_response.error {
        return Err(format!("API Error: {}", error.message).into());
    }

    if let Some(choice) = deepseek_response.choices.first() {
        return Ok(choice.message.content.clone());
    }

    Err("No response from API".into())
}
