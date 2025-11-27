use crate::features::service::constants::DEFAULT_GEMINI_MODEL;
use crate::features::service::models::gemini::types::{
    ContentPart, GeminiRequest, GeminiResponse, TextPart,
};
use reqwest::Client;

pub async fn gemini(
    prompt: &str,
    api_key: &str,
    model: &str,
) -> Result<String, Box<dyn std::error::Error>> {
    let resolved_model = if model.trim().is_empty() {
        DEFAULT_GEMINI_MODEL.to_string()
    } else {
        model.to_string()
    };

    // Build request payload
    let request = GeminiRequest {
        contents: vec![ContentPart {
            parts: vec![TextPart {
                text: prompt.to_string(),
            }],
        }],
    };

    // Create HTTP client and send request
    let client = Client::new();
    let url = format!(
        "https://generativelanguage.googleapis.com/v1beta/models/{}:generateContent?key={}",
        resolved_model, api_key
    );

    let response = client
        .post(&url)
        .header("Content-Type", "application/json")
        .json(&request)
        .send()
        .await?;

    // Parse response
    let gemini_response: GeminiResponse = response.json().await?;

    // Handle errors
    if let Some(error) = gemini_response.error {
        return Err(format!("API Error: {}", error.message).into());
    }

    // Extract text from response
    if let Some(candidates) = gemini_response.candidates
        && let Some(candidate) = candidates.first()
        && let Some(part) = candidate.content.parts.first()
    {
        return Ok(part.text.clone());
    }

    Err("No response from API".into())
}
