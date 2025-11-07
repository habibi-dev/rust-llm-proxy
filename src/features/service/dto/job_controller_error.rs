use axum::http::StatusCode;
use sea_orm::DbErr;
use serde::Serialize;

#[derive(Debug, Serialize)]
pub enum JobControllerError {
    ApiKeyNotFound,
    Validation(String),
    JobCreation,
    Repository(String),
}

impl JobControllerError {
    pub fn message(&self) -> String {
        match self {
            JobControllerError::ApiKeyNotFound => "API key not found".to_string(),
            JobControllerError::Validation(err) => err.clone(),
            JobControllerError::JobCreation => "Unable to create job".to_string(),
            JobControllerError::Repository(err) => err.clone(),
        }
    }

    pub fn status_code(&self) -> StatusCode {
        match self {
            JobControllerError::ApiKeyNotFound => StatusCode::UNAUTHORIZED,
            JobControllerError::Validation(_) => StatusCode::BAD_REQUEST,
            JobControllerError::JobCreation | JobControllerError::Repository(_) => {
                StatusCode::INTERNAL_SERVER_ERROR
            }
        }
    }
}

impl From<DbErr> for JobControllerError {
    fn from(value: DbErr) -> Self {
        JobControllerError::Repository(value.to_string())
    }
}
