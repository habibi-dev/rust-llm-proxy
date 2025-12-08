use serde::Deserialize;
use validator::Validate;

#[derive(Debug, Deserialize, Validate)]
pub struct ServiceForm {
    pub title: String,
    pub provider: String,
    pub key: Option<String>,
    pub model: Option<String>,
    pub status: Option<bool>,
    #[serde(default)]
    pub deepseek_settings: Option<DeepSeekSettingsForm>,
}

impl ServiceForm {
    pub fn build_settings(&self) -> Result<ServiceSettings, String> {
        let mut settings = ServiceSettings::default();

        if let Some(deepseek_settings) = &self.deepseek_settings {
            settings.deepseek = Some(deepseek_settings.to_settings()?);
        }

        Ok(settings)
    }
}

#[derive(Debug, Deserialize, Validate, Default)]
pub struct DeepSeekSettingsForm {
    pub frequency_penalty: Option<f32>,
    pub max_tokens: Option<u32>,
    pub presence_penalty: Option<f32>,
    pub response_format: Option<String>,
    pub stop: Option<String>,
    pub stream_options: Option<String>,
    pub temperature: Option<f32>,
    pub top_p: Option<f32>,
    pub tools: Option<String>,
    pub tool_choice: Option<String>,
    pub logprobs: Option<bool>,
    pub top_logprobs: Option<u8>,
    pub thinking: Option<String>,
}

impl DeepSeekSettingsForm {
    pub fn to_settings(&self) -> Result<DeepSeekSettings, String> {
        self.validate_ranges()?;

        Ok(DeepSeekSettings {
            frequency_penalty: self.frequency_penalty,
            max_tokens: self.max_tokens,
            presence_penalty: self.presence_penalty,
            response_format: Self::parse_json(&self.response_format, "response_format")?,
            stop: Self::parse_json(&self.stop, "stop")?,
            stream_options: Self::parse_json(&self.stream_options, "stream_options")?,
            temperature: self.temperature,
            top_p: self.top_p,
            tools: Self::parse_json(&self.tools, "tools")?,
            tool_choice: Self::parse_json(&self.tool_choice, "tool_choice")?,
            logprobs: self.logprobs,
            top_logprobs: self.top_logprobs,
            thinking: Self::parse_json(&self.thinking, "thinking")?,
        })
    }

    fn parse_json(value: &Option<String>, field: &str) -> Result<Option<Value>, String> {
        match value {
            Some(raw) if !raw.trim().is_empty() => {
                serde_json::from_str(raw).map_err(|err| format!("Invalid {field} JSON: {err}"))
            }
            _ => Ok(None),
        }
    }

    fn validate_ranges(&self) -> Result<(), String> {
        if let Some(value) = self.frequency_penalty {
            if !((-2.0..=2.0).contains(&value)) {
                return Err("frequency_penalty must be between -2 and 2".to_string());
            }
        }

        if let Some(value) = self.presence_penalty {
            if !((-2.0..=2.0).contains(&value)) {
                return Err("presence_penalty must be between -2 and 2".to_string());
            }
        }

        if let Some(value) = self.temperature {
            if value < 0.0 || value > 2.0 {
                return Err("temperature must be between 0 and 2".to_string());
            }
        }

        if let Some(value) = self.top_p {
            if value < 0.0 || value > 1.0 {
                return Err("top_p must be between 0 and 1".to_string());
            }
        }

        if let Some(value) = self.top_logprobs {
            if value > 20 {
                return Err("top_logprobs must be 20 or less".to_string());
            }
        }

        Ok(())
    }
}
use crate::features::service::dto::service_settings::{DeepSeekSettings, ServiceSettings};
use serde_json::Value;
