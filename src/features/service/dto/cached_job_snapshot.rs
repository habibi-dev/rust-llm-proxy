use crate::features::service::dto::provider_response::{ProviderResponse, UsageMetrics};
use crate::features::service::model::jobs;
use crate::features::service::model::jobs::JobStatus;
use sea_orm::prelude::Json;

pub struct CachedJobSnapshot {
    pub parent_job_id: i64,
    pub status: JobStatus,
    pub output: Option<Json>,
    pub result: Option<ProviderResponse>,
    pub error: Option<String>,
}

impl CachedJobSnapshot {
    pub fn try_from(model: &jobs::Model) -> Option<Self> {
        if !matches!(model.status, JobStatus::Done | JobStatus::Failed) {
            return None;
        }

        let output = model.output.clone();
        let (result, error) = Self::extract_payload(&output);

        if result.is_none() && error.is_none() {
            return None;
        }

        Some(Self {
            parent_job_id: model.id,
            status: model.status.clone(),
            output,
            result,
            error,
        })
    }

    pub fn extract_payload(output: &Option<Json>) -> (Option<ProviderResponse>, Option<String>) {
        let Some(value) = output.as_ref() else {
            return (None, None);
        };

        let result = value.get("response").and_then(|data| {
            let message = data
                .get("message")
                .and_then(|value| value.as_str())
                .map(|content| content.to_string());

            let usage = data.get("usage").map(|usage_value| UsageMetrics {
                completion_tokens: usage_value
                    .get("completion_tokens")
                    .and_then(|value| value.as_i64()),
                prompt_tokens: usage_value
                    .get("prompt_tokens")
                    .and_then(|value| value.as_i64()),
                total_tokens: usage_value
                    .get("total_tokens")
                    .and_then(|value| value.as_i64()),
            });

            message.map(|content| ProviderResponse {
                message: content,
                usage,
            })
        });

        let error = value
            .get("error")
            .and_then(|data| data.as_str())
            .map(|content| content.to_string());

        (result, error)
    }
}
