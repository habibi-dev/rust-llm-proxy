use crate::features::service::constants::DEFAULT_DEEPSEEK_MODEL;
use crate::features::service::dto::chat_prompt::ChatPrompt;
use crate::features::service::dto::provider_response::{ProviderResponse, UsageMetrics};
use crate::features::service::dto::service_settings::ServiceSettings;
use crate::features::service::models::deepseek::types::{
    ChatMessage, DeepSeekRequest, DeepSeekResponse,
};
use reqwest::Client;
use serde_json::from_str;

pub async fn deepseek(
    prompt: &ChatPrompt,
    api_key: &str,
    model: &str,
    settings: &ServiceSettings,
) -> Result<ProviderResponse, Box<dyn std::error::Error>> {
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

    let request = DeepSeekRequest {
        model: resolved_model,
        messages,
        stream: false,
        frequency_penalty: settings
            .deepseek
            .as_ref()
            .and_then(|config| config.frequency_penalty),
        max_tokens: settings
            .deepseek
            .as_ref()
            .and_then(|config| config.max_tokens),
        presence_penalty: settings
            .deepseek
            .as_ref()
            .and_then(|config| config.presence_penalty),
        response_format: settings
            .deepseek
            .as_ref()
            .and_then(|config| config.response_format.clone()),
        stop: settings
            .deepseek
            .as_ref()
            .and_then(|config| config.stop.clone()),
        stream_options: settings
            .deepseek
            .as_ref()
            .and_then(|config| config.stream_options.clone()),
        temperature: settings
            .deepseek
            .as_ref()
            .and_then(|config| config.temperature),
        top_p: settings.deepseek.as_ref().and_then(|config| config.top_p),
        tools: settings
            .deepseek
            .as_ref()
            .and_then(|config| config.tools.clone()),
        tool_choice: settings
            .deepseek
            .as_ref()
            .and_then(|config| config.tool_choice.clone()),
        logprobs: settings
            .deepseek
            .as_ref()
            .and_then(|config| config.logprobs),
        top_logprobs: settings
            .deepseek
            .as_ref()
            .and_then(|config| config.top_logprobs),
        thinking: settings
            .deepseek
            .as_ref()
            .and_then(|config| config.thinking.clone()),
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
        let usage = deepseek_response.usage.map(|metrics| {
            UsageMetrics::new(
                metrics.completion_tokens,
                metrics.prompt_tokens,
                metrics.total_tokens,
            )
        });

        return Ok(ProviderResponse::new(choice.message.content.clone(), usage));
    }

    Err("No response from API".into())
}
