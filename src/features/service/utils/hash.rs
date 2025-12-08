use crate::features::service::dto::chat_prompt::ChatPrompt;
use crate::features::service::model::service;
use blake3::Hasher;
use chrono::Local;

pub fn generate(prompt: &ChatPrompt, service: &service::Model) -> String {
    let mut hasher = Hasher::new();

    // Base inputs
    hasher.update(service.provider.as_bytes());
    hasher.update(service.key.as_bytes());
    hasher.update(prompt.user_message.as_bytes());

    // Optional system message
    if let Some(system) = prompt.system_message.as_deref() {
        hasher.update(system.as_bytes());
    }

    // Current time rounded to minute
    let now_minute = Local::now().format("%Y-%m-%d %H:%M").to_string();
    hasher.update(now_minute.as_bytes());

    hasher.finalize().to_hex().to_string()
}
