use crate::features::service::dto::chat_prompt::ChatPrompt;
use serde_json::{Value as Json, json};

pub struct JobPayloadBuilder;

impl JobPayloadBuilder {
    pub fn build_input(prompt: &ChatPrompt) -> Json {
        json!({
            "message": prompt.user_message.clone(),
            "system": prompt.system_message.clone(),
        })
    }

    pub fn build_success_output(result: &str) -> Json {
        json!({ "response": result })
    }

    pub fn build_error_output(error: &str) -> Json {
        json!({ "error": error })
    }
}
