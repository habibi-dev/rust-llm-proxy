use crate::features::service::models::gemini;

pub struct ProviderDispatcher;

impl ProviderDispatcher {
    pub async fn dispatch(provider: &str, message: &str, key: &str) -> Result<String, String> {
        match provider {
            "gemini" => gemini::main::gemini(message, key)
                .await
                .map_err(|err| err.to_string()),
            _ => Err("Unknown provider".to_string()),
        }
    }
}
