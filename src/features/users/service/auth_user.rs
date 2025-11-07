use crate::features::users::model::user;
use axum::extract::FromRequestParts;
use axum::http::StatusCode;
use axum::http::request::Parts;
use std::sync::Arc;

pub struct AuthUser(pub Arc<user::Model>);

impl<S> FromRequestParts<S> for AuthUser
where
    S: Send + Sync,
{
    type Rejection = (StatusCode, String);

    async fn from_request_parts(parts: &mut Parts, _state: &S) -> Result<Self, Self::Rejection> {
        parts
            .extensions
            .get::<Arc<user::Model>>()
            .cloned()
            .map(AuthUser)
            .ok_or((StatusCode::UNAUTHORIZED, "Not authenticated".to_string()))
    }
}
