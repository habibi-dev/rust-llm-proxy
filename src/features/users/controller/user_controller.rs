use crate::core::response::{json_error, json_success};
use crate::features::users::repository::users_repo::UsersRepository;
use crate::features::users::service::auth_user::AuthUser;
use crate::features::users::validation::user_form::UserForm;
use axum::Form;
use axum::extract::Path;
use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};
use validator::Validate;

pub struct UserController;
impl UserController {
    pub async fn user_list() -> impl IntoResponse {
        let users = UsersRepository::all().await.unwrap_or(Vec::new());
        json_success(users)
    }

    pub async fn user_create(Form(form): Form<UserForm>) -> Response {
        if let Err(e) = form.validate() {
            return json_error(StatusCode::BAD_REQUEST, e.to_string());
        }

        match UsersRepository::create(form).await {
            Ok(user) => json_success(user),
            Err(message) => json_error(StatusCode::BAD_REQUEST, message.to_string()),
        }
    }

    pub async fn user_update(
        AuthUser(user): AuthUser,
        Path(user_id): Path<i64>,
        Form(form): Form<UserForm>,
    ) -> Response {
        if let Err(e) = form.validate() {
            return json_error(StatusCode::BAD_REQUEST, e.to_string());
        }

        // Check if the user is trying to update their own account admin status
        if user.id == user_id && !form.is_admin.unwrap_or(false) {
            return json_error(
                StatusCode::BAD_REQUEST,
                "You cannot change your own admin status.",
            );
        }

        match UsersRepository::update(user_id, form).await {
            Ok(user) => json_success(user),
            Err(message) => json_error(StatusCode::BAD_REQUEST, message.to_string()),
        }
    }

    pub async fn user_delete(Path(user_id): Path<i64>, AuthUser(user): AuthUser) -> Response {
        // Check if the user is trying to delete their own account
        if user.id == user_id {
            return json_error(
                StatusCode::BAD_REQUEST,
                "You cannot delete your own account.",
            );
        }

        match UsersRepository::delete(user_id).await {
            Ok(result) => json_success(result),
            Err(message) => json_error(StatusCode::BAD_REQUEST, message.to_string()),
        }
    }

    pub async fn me(AuthUser(user): AuthUser) -> Response {
        json_success(user)
    }
}
