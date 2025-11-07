use crate::features::service::model::jobs::JobStatus;
use sea_orm::prelude::{DateTime, Json};
use serde::Deserialize;
use validator::Validate;

#[derive(Debug, Deserialize, Validate)]
pub struct JobForm {
    pub hash: String,
    pub model: String,
    pub input: Json,
    pub output: Option<Json>,
    pub user_id: i64,
    pub api_key_id: i64,
    #[validate(range(min = 1, max = 10))]
    pub retry: i8,
    pub status: JobStatus,
    pub finished_at: Option<DateTime>,
}
