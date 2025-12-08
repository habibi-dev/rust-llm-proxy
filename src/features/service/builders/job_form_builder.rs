use super::job_payload_builder::JobPayloadBuilder;
use crate::features::service::dto::chat_prompt::ChatPrompt;
use crate::features::service::model::jobs::JobStatus;
use crate::features::service::model::service;
use crate::features::service::validation::job_form::JobForm;
use sea_orm::prelude::{DateTime, Json};

pub struct JobFormBuilderConfig {
    pub hash: String,
    pub service_model: service::Model,
    pub user_id: i64,
    pub api_key_id: i64,
    pub prompt: ChatPrompt,
    pub status: JobStatus,
    pub output: Option<Json>,
    pub finished_at: Option<DateTime>,
}

pub struct JobFormBuilder;

impl JobFormBuilder {
    pub fn build(config: JobFormBuilderConfig) -> JobForm {
        JobForm {
            hash: config.hash,
            model: config.service_model.model.clone(),
            input: JobPayloadBuilder::build_input(&config.prompt),
            output: config.output,
            user_id: config.user_id,
            api_key_id: config.api_key_id,
            retry: 1,
            status: config.status,
            finished_at: config.finished_at,
        }
    }
}
