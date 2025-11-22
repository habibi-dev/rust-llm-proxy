use crate::core::response::{json_error, json_success};
use crate::features::service::builders::JobFormBuilder;
use crate::features::service::builders::job_form_builder::JobFormBuilderConfig;
use crate::features::service::dto::job_chat_response::JobChatResponse;
use crate::features::service::dto::job_controller_error::JobControllerError;
use crate::features::service::dto::job_execution_context::JobExecutionContext;
use crate::features::service::model::jobs::JobStatus;
use crate::features::service::model::service;
use crate::features::service::repository::jobs_repository::JobRepository;
use crate::features::service::service_job::{JobCacheManager, JobExecutor};
use crate::features::service::utils::hash;
use crate::features::users::model::user;
use crate::features::users::repository::repo_api_key::RepositoryApiKey;
use crate::features::users::service::auth_user::AuthUser;
use axum::extract::Path;
use axum::http::StatusCode;
use axum::response::Response;
use tokio::sync::oneshot;
use tokio::time::{Duration, timeout};
use validator::Validate;

pub struct JobController;

impl JobController {
    pub async fn handle_chat_request(
        service: &service::Model,
        user: &user::Model,
        raw_api_key: &str,
        message: &str,
    ) -> Result<JobChatResponse, JobControllerError> {
        let Some(api_key) = RepositoryApiKey::get_by_key(raw_api_key).await else {
            return Err(JobControllerError::ApiKeyNotFound);
        };

        let hash = hash::generate(message, service);

        // Try to use cached response
        if let Some(cached_response) =
            JobCacheManager::try_get_cached_response(&hash, service, user.id, api_key.id, message)
                .await?
        {
            return Ok(cached_response);
        }

        let data = JobFormBuilderConfig {
            hash,
            service_model: service.clone(),
            user_id: user.id,
            api_key_id: api_key.id,
            message: message.to_string(),
            status: JobStatus::Queued,
            output: None,
            finished_at: None,
        };

        // Create new job
        let job_form = JobFormBuilder::build(data);

        job_form
            .validate()
            .map_err(|err| JobControllerError::Validation(err.to_string()))?;

        let job = JobRepository::create(job_form)
            .await
            .map_err(JobControllerError::from)?
            .ok_or(JobControllerError::JobCreation)?;

        let job_id = job.id;
        let (sender, receiver) = oneshot::channel();
        let context = JobExecutionContext::new(
            job_id,
            service.provider.clone(),
            service.key.clone(),
            message.to_owned(),
        );

        // Spawn job execution
        tokio::spawn(async move {
            let result = JobExecutor::execute(context).await;
            sender.send(result).ok();
        });

        // Wait for result with timeout
        match timeout(Duration::from_secs(59), receiver).await {
            Ok(Ok(Ok(result))) => Ok(JobChatResponse::completed(job_id, result)),
            Ok(Ok(Err(error))) => Ok(JobChatResponse::failed(job_id, error)),
            Ok(Err(_canceled)) => Ok(JobChatResponse::failed(
                job_id,
                "Job channel closed before completion".to_string(),
            )),
            Err(_) => Ok(JobChatResponse::processing(job_id)),
        }
    }

    pub async fn show(Path(job_id): Path<i64>, AuthUser(user): AuthUser) -> Response {
        let job = JobRepository::find_by_id(job_id).await.unwrap_or(None);

        if job.is_none() {
            return json_error(StatusCode::NOT_FOUND, "Job not found".to_string());
        }

        if job.as_ref().unwrap().user_id != user.id {
            return json_error(
                StatusCode::FORBIDDEN,
                "User does not match the provided user",
            );
        }

        json_success(job)
    }
}
