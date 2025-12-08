use super::provider_response::ProviderResponse;
use crate::features::service::model::jobs::JobStatus;
use chrono::{DateTime as ChronoDateTime, Utc};
use serde::Serialize;

#[derive(Debug, Serialize)]
pub struct JobChatResponse {
    pub job_id: i64,
    pub status: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub result: Option<ProviderResponse>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parent_job_id: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub responded_at: Option<String>,
}

impl JobChatResponse {
    pub fn completed(job_id: i64, result: ProviderResponse) -> Self {
        let now = Utc::now();
        Self {
            job_id,
            status: Self::status_from_job_status(&JobStatus::Done).to_string(),
            result: Some(result),
            error: None,
            parent_job_id: None,
            responded_at: Some(now.to_rfc3339()),
        }
    }

    pub fn processing(job_id: i64) -> Self {
        Self {
            job_id,
            status: Self::status_from_job_status(&JobStatus::Running).to_string(),
            result: None,
            error: None,
            parent_job_id: None,
            responded_at: None,
        }
    }

    pub fn failed(job_id: i64, error: String) -> Self {
        let now = Utc::now();
        Self {
            job_id,
            status: Self::status_from_job_status(&JobStatus::Failed).to_string(),
            result: None,
            error: Some(error),
            parent_job_id: None,
            responded_at: Some(now.to_rfc3339()),
        }
    }

    pub fn from_cached(
        job_id: i64,
        parent_job_id: i64,
        status: JobStatus,
        result: Option<ProviderResponse>,
        error: Option<String>,
        responded_at: ChronoDateTime<Utc>,
    ) -> Self {
        Self {
            job_id,
            status: Self::status_from_job_status(&status).to_string(),
            result,
            error,
            parent_job_id: Some(parent_job_id),
            responded_at: Some(responded_at.to_rfc3339()),
        }
    }

    pub fn status_from_job_status(status: &JobStatus) -> &'static str {
        match status {
            JobStatus::Queued | JobStatus::Running => "processing",
            JobStatus::Done => "completed",
            JobStatus::Failed => "failed",
        }
    }
}
