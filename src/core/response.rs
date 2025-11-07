use axum::Json;
use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};
use serde::Serialize;

#[derive(Serialize)]
pub struct ApiResponse<T> {
    pub success: bool,
    pub data: Option<T>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error: Option<ErrorDetail>,
}

#[derive(Serialize)]
pub struct ErrorDetail {
    pub code: String,
    pub message: String,
}

impl<T> ApiResponse<T> {
    pub fn success(data: T) -> Self {
        Self {
            success: true,
            data: Some(data),
            error: None,
        }
    }

    pub fn error(code: impl Into<String>, message: impl Into<String>) -> Self {
        Self {
            success: false,
            data: None,
            error: Some(ErrorDetail {
                code: code.into(),
                message: message.into(),
            }),
        }
    }
}

pub fn json_success<T: Serialize>(data: T) -> Response {
    (StatusCode::OK, Json(ApiResponse::success(data))).into_response()
}

pub fn json_error(code: StatusCode, msg: impl Into<String>) -> Response {
    let err = ApiResponse::<()>::error(code.as_u16().to_string(), msg.into());
    (code, Json(err)).into_response()
}
