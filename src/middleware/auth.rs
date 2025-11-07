use crate::core::response::json_error;
use crate::core::state::AppState;
use crate::features::users::service::srv_api_key::ServiceApiKey;
use axum::extract::{Request, State};
use axum::http::StatusCode;
use axum::middleware::Next;
use axum::response::{IntoResponse, Response};

pub async fn auth(State(_state): State<AppState>, mut req: Request, next: Next) -> Response {
    let key = match req.headers().get("X-API-Key").and_then(|v| v.to_str().ok()) {
        Some(k) => k,
        None => return json_error(StatusCode::UNAUTHORIZED, "Missing API key").into_response(),
    };

    if key.is_empty() {
        return json_error(StatusCode::UNAUTHORIZED, "Unauthorized").into_response();
    }

    if let Some(user) = ServiceApiKey::auth(key).await {
        req.extensions_mut().insert(std::sync::Arc::new(user));
    } else {
        return json_error(StatusCode::UNAUTHORIZED, "Invalid API key").into_response();
    }

    next.run(req).await
}
