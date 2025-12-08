use crate::features::service::dto::chat_prompt::ChatPrompt;
use crate::features::service::models::{deepseek, gemini};

pub struct ProviderDispatcher;

impl ProviderDispatcher {
    pub async fn dispatch(
        provider: &str,
        model: &str,
        prompt: &ChatPrompt,
        key: &str,
    ) -> Result<String, String> {
        match provider {
            "gemini" => gemini::main::gemini(prompt, key, model)
                .await
                .map_err(|err| err.to_string()),
            "deepseek" => deepseek::main::deepseek(prompt, key, model)
                .await
                .map_err(|err| err.to_string()),
            _ => Err("Unknown provider".to_string()),
        }
    }
}
