use crate::core::state::AppState;
use crate::features::users::service::srv_api_key::ServiceApiKey;
use axum::extract::{Request, State};
use axum::http::StatusCode;
use axum::middleware::Next;
use axum::response::{IntoResponse, Response};

pub async fn is_admin(State(_state): State<AppState>, req: Request, next: Next) -> Response {
    let key = match req.headers().get("X-API-Key").and_then(|v| v.to_str().ok()) {
        Some(k) => k,
        None => return (StatusCode::UNAUTHORIZED, "Missing API key").into_response(),
    };

    if key.is_empty() {
        return (StatusCode::UNAUTHORIZED, "Unauthorized").into_response();
    }

    let is_admin = ServiceApiKey::is_admin(key).await;

    if !is_admin {
        return (StatusCode::FORBIDDEN, "Access Denied").into_response();
    }

    next.run(req).await
}
