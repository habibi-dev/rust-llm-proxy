use crate::features::service::dto::chat_prompt::ChatPrompt;
use crate::features::service::model::service;
use blake3::Hasher;

pub fn generate(prompt: &ChatPrompt, service: &service::Model) -> String {
    let mut hasher = Hasher::new();
    hasher.update(service.provider.as_bytes());
    hasher.update(service.key.as_bytes());
    hasher.update(prompt.user_message.as_bytes());
    if let Some(system) = &prompt.system_message {
        hasher.update(system.as_bytes());
    }
    hasher.finalize().to_hex().to_string()
}
