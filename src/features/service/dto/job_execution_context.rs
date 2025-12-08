use super::chat_prompt::ChatPrompt;
use super::service_settings::ServiceSettings;

pub struct JobExecutionContext {
    pub job_id: i64,
    pub provider: String,
    pub model: String,
    pub key: String,
    pub prompt: ChatPrompt,
    pub settings: ServiceSettings,
}

impl JobExecutionContext {
    pub fn new(
        job_id: i64,
        provider: String,
        model: String,
        key: String,
        prompt: ChatPrompt,
        settings: ServiceSettings,
    ) -> Self {
        Self {
            job_id,
            provider,
            model,
            key,
            prompt,
            settings,
        }
    }
}
