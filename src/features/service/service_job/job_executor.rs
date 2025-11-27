use crate::features::service::builders::JobPayloadBuilder;
use crate::features::service::dto::job_execution_context::JobExecutionContext;
use crate::features::service::model::jobs::{JobStatus, Model};
use crate::features::service::repository::jobs_repository::JobRepository;
use crate::features::service::service_job::provider_dispatcher::ProviderDispatcher;
use chrono::Utc;
use sea_orm::prelude::Json;

pub struct JobExecutor;

impl JobExecutor {
    pub async fn execute(context: JobExecutionContext) -> Result<String, String> {
        Self::update_status(context.job_id, JobStatus::Running, None).await?;

        match ProviderDispatcher::dispatch(
            &context.provider,
            &context.model,
            &context.message,
            &context.key,
        )
        .await
        {
            Ok(output) => {
                Self::update_status(
                    context.job_id,
                    JobStatus::Done,
                    Some(JobPayloadBuilder::build_success_output(&output)),
                )
                .await
                .ok();
                Ok(output)
            }
            Err(error) => {
                Self::update_status(
                    context.job_id,
                    JobStatus::Failed,
                    Some(JobPayloadBuilder::build_error_output(&error)),
                )
                .await
                .ok();
                Err(error)
            }
        }
    }

    async fn update_status(
        job_id: i64,
        status: JobStatus,
        output: Option<Json>,
    ) -> Result<Option<Model>, String> {
        let finished_at = Utc::now().naive_utc();
        JobRepository::update_output_status(job_id, output, status, Some(finished_at))
            .await
            .map_err(|err| format!("Failed to update job status: {}", err))
    }
}
