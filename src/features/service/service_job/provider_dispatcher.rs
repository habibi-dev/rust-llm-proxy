use crate::features::service::dto::chat_prompt::ChatPrompt;
use crate::features::service::dto::provider_response::ProviderResponse;
use crate::features::service::dto::service_settings::ServiceSettings;
use crate::features::service::models::{deepseek, gemini};

pub struct ProviderDispatcher;

impl ProviderDispatcher {
    pub async fn dispatch(
        provider: &str,
        model: &str,
        prompt: &ChatPrompt,
        key: &str,
        settings: &ServiceSettings,
    ) -> Result<ProviderResponse, String> {
        match provider {
            "gemini" => gemini::main::gemini(prompt, key, model)
                .await
                .map(|message| ProviderResponse::new(message, None))
                .map_err(|err| err.to_string()),
            "deepseek" => deepseek::main::deepseek(prompt, key, model, settings)
                .await
                .map_err(|err| err.to_string()),
            _ => Err("Unknown provider".to_string()),
        }
    }
}
