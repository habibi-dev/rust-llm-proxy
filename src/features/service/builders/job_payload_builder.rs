use crate::features::service::dto::chat_prompt::ChatPrompt;
use crate::features::service::dto::provider_response::ProviderResponse;
use serde_json::{Value as Json, json};

pub struct JobPayloadBuilder;

impl JobPayloadBuilder {
    pub fn build_input(prompt: &ChatPrompt) -> Json {
        json!({
            "message": prompt.user_message.clone(),
            "system": prompt.system_message.clone(),
        })
    }

    pub fn build_success_output(result: &ProviderResponse) -> Json {
        json!({
            "response": {
                "message": result.message,
                "usage": result.usage,
            }
        })
    }

    pub fn build_error_output(error: &str) -> Json {
        json!({ "error": error })
    }
}
