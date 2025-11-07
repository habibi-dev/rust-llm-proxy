use crate::core::response::{json_error, json_success};
use crate::features::users::repository::repo_api_key::RepositoryApiKey;
use crate::features::users::repository::users_repo::UsersRepository;
use crate::features::users::service::auth_user::AuthUser;
use crate::features::users::utility::hash_key::hash_key;
use crate::features::users::utility::key_generator::key_generator;
use crate::features::users::validation::api_key_form::ApiKeyForm;
use crate::utility::state::app_state;
use axum::Form;
use axum::extract::Path;
use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};
use validator::Validate;

pub struct ApiKeyController;
impl ApiKeyController {
    pub async fn list() -> impl IntoResponse {
        let api_keys = RepositoryApiKey::all().await.unwrap_or(Vec::new());
        json_success(api_keys)
    }

    pub async fn create(AuthUser(user): AuthUser, Form(mut form): Form<ApiKeyForm>) -> Response {
        if let Err(e) = form.validate() {
            return json_error(StatusCode::BAD_REQUEST, e.to_string());
        }

        // Default to authenticated user's ID
        let target_user_id = form.user_id.unwrap_or(user.id);

        // Verify target user exists
        if target_user_id != user.id {
            let state = app_state();
            match UsersRepository::find_by_id(&state._db, target_user_id).await {
                Ok(Some(_)) => (),
                Ok(None) => {
                    return json_error(StatusCode::NOT_FOUND, "User not found");
                }
                Err(e) => {
                    return json_error(
                        StatusCode::INTERNAL_SERVER_ERROR,
                        format!("Database error: {}", e),
                    );
                }
            }
        }

        // Generate and hash key
        let raw_key = key_generator();
        let key_hash = hash_key(&raw_key);

        form.user_id = Some(target_user_id);
        form.key = Some(raw_key);
        form.key_hash = Some(key_hash);

        match RepositoryApiKey::create(form).await {
            Ok(api_key) => {
                let mut response = serde_json::json!(api_key.0);
                response["key"] = serde_json::json!(api_key.1);
                json_success(response)
            }
            Err(e) => json_error(StatusCode::INTERNAL_SERVER_ERROR, e.to_string()),
        }
    }

    pub async fn update(Path(api_key): Path<String>, Form(form): Form<ApiKeyForm>) -> Response {
        if let Err(e) = form.validate() {
            return json_error(StatusCode::BAD_REQUEST, e.to_string());
        }

        match RepositoryApiKey::update_status(
            api_key.as_str(),
            form.status.expect("Status is required!"),
        )
        .await
        {
            Ok(status) => json_success(serde_json::json!({ "updated": status })),
            Err(message) => json_error(StatusCode::BAD_REQUEST, message.to_string()),
        }
    }

    pub async fn delete(Path(api_key): Path<String>) -> Response {
        match RepositoryApiKey::delete(api_key.as_str()).await {
            Ok(result) => json_success(result),
            Err(message) => json_error(StatusCode::BAD_REQUEST, message.to_string()),
        }
    }
}
