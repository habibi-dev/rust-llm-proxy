use super::job_controller::JobController;
use crate::core::response::{json_error, json_success};
use crate::features::service::repository::service_repository::ServiceRepository;
use crate::features::service::validation::chat_form::ChatForm;
use crate::features::service::validation::service_form::ServiceForm;
use crate::features::users::service::auth_user::AuthUser;
use axum::Form;
use axum::extract::Path;
use axum::http::{HeaderMap, StatusCode};
use axum::response::{IntoResponse, Response};
use validator::Validate;

pub struct ServiceController;
impl ServiceController {
    pub async fn list() -> impl IntoResponse {
        let services = ServiceRepository::list().await.unwrap_or(Vec::new());
        json_success(services)
    }

    pub async fn create(Form(form): Form<ServiceForm>) -> Response {
        if let Err(e) = form.validate() {
            return json_error(StatusCode::BAD_REQUEST, e.to_string());
        }

        match ServiceRepository::create(form).await {
            Ok(response) => json_success(response),
            Err(e) => json_error(StatusCode::INTERNAL_SERVER_ERROR, e.to_string()),
        }
    }

    pub async fn update(Path(service_id): Path<i64>, Form(form): Form<ServiceForm>) -> Response {
        if let Err(e) = form.validate() {
            return json_error(StatusCode::BAD_REQUEST, e.to_string());
        }

        match ServiceRepository::update(service_id, form).await {
            Ok(service) => match service {
                Some(service) => json_success(service),
                None => json_error(StatusCode::NOT_FOUND, "Service not found".to_string()),
            },
            Err(message) => json_error(StatusCode::BAD_REQUEST, message.to_string()),
        }
    }

    pub async fn chat(
        headers: HeaderMap,
        AuthUser(user): AuthUser,
        Path(service_id): Path<i64>,
        Form(form): Form<ChatForm>,
    ) -> Response {
        if let Err(e) = form.validate() {
            return json_error(StatusCode::BAD_REQUEST, e.to_string());
        }

        let service = match ServiceRepository::find_by_id(service_id).await {
            Ok(Some(s)) => s,
            Ok(None) => return json_error(StatusCode::NOT_FOUND, "Service not found"),
            Err(e) => return json_error(StatusCode::BAD_REQUEST, e.to_string()),
        };

        if !service.status {
            return json_error(StatusCode::NOT_FOUND, "Service is inactive");
        }

        let Some(raw_api_key) = headers
            .get("X-API-Key")
            .and_then(|value| value.to_str().ok())
        else {
            return json_error(StatusCode::UNAUTHORIZED, "Missing API key");
        };

        match JobController::handle_chat_request(
            &service,
            user.as_ref(),
            raw_api_key,
            &form.message,
        )
        .await
        {
            Ok(payload) => json_success(payload),
            Err(err) => json_error(err.status_code(), err.message()),
        }
    }

    pub async fn delete(Path(service_id): Path<i64>) -> Response {
        match ServiceRepository::delete(service_id).await {
            Ok(result) => json_success(result),
            Err(message) => json_error(StatusCode::BAD_REQUEST, message.to_string()),
        }
    }
}
