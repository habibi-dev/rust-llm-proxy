use crate::features::service::builders::JobFormBuilder;
use crate::features::service::builders::job_form_builder::JobFormBuilderConfig;
use crate::features::service::dto::cached_job_snapshot::CachedJobSnapshot;
use crate::features::service::dto::job_chat_response::JobChatResponse;
use crate::features::service::dto::job_controller_error::JobControllerError;
use crate::features::service::model::service;
use crate::features::service::repository::jobs_repository::JobRepository;
use crate::features::service::utils::unique_hash::generate;
use chrono::Utc;
use validator::Validate;

pub struct JobCacheManager;

impl JobCacheManager {
    pub async fn try_get_cached_response(
        hash: &str,
        service: &service::Model,
        user_id: i64,
        api_key_id: i64,
        message: &str,
    ) -> Result<Option<JobChatResponse>, JobControllerError> {
        let Some(existing_job) = JobRepository::find_by_hash(hash)
            .await
            .map_err(JobControllerError::from)?
        else {
            return Ok(None);
        };

        let Some(snapshot) = CachedJobSnapshot::try_from(&existing_job) else {
            return Ok(None);
        };

        let data = JobFormBuilderConfig {
            hash: generate(hash),
            service_model: service.clone(),
            user_id,
            api_key_id,
            message: message.to_string(),
            status: snapshot.status.clone(),
            output: snapshot.output.clone(),
            finished_at: Some(Utc::now().naive_utc()),
        };

        let job_form = JobFormBuilder::build(data);

        job_form
            .validate()
            .map_err(|err| JobControllerError::Validation(err.to_string()))?;

        let job = JobRepository::create(job_form)
            .await
            .map_err(JobControllerError::from)?
            .ok_or(JobControllerError::JobCreation)?;

        let now = Utc::now();
        Ok(Some(JobChatResponse::from_cached(
            job.id,
            snapshot.parent_job_id,
            snapshot.status.clone(),
            snapshot.result.clone(),
            snapshot.error.clone(),
            now,
        )))
    }
}
