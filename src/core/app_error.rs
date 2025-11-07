use crate::core::response::json_error;
use axum::body::{Body, to_bytes};
use axum::extract::Request;
use axum::middleware::Next;
use axum::response::IntoResponse;
use axum::{http::StatusCode, response::Response};
use tower::BoxError;

pub async fn handle_tower_error(err: BoxError) -> Response {
    if err.is::<tower::timeout::error::Elapsed>() {
        return json_error(StatusCode::REQUEST_TIMEOUT, "Request timed out");
    }
    json_error(
        StatusCode::INTERNAL_SERVER_ERROR,
        format!("Internal error: {err}"),
    )
}

pub async fn handle_normalize_error(req: Request<Body>, next: Next) -> Response {
    let res = next.run(req).await;

    // Check if response is an error
    let status = res.status();

    // If it's a 4xx or 5xx and not JSON, normalize it
    if status.is_client_error() || status.is_server_error() {
        let content_type = res
            .headers()
            .get("content-type")
            .and_then(|v| v.to_str().ok())
            .unwrap_or("");

        if !content_type.contains("application/json") {
            // Read body bytes
            let body_bytes = to_bytes(res.into_body(), usize::MAX)
                .await
                .unwrap_or_default();
            let mut message = String::from_utf8_lossy(&body_bytes);

            if message.is_empty() {
                message = default_message(status).into();
            }

            // Return normalized JSON error
            return json_error(status, message).into_response();
        }
    }

    res
}

fn default_message(status: StatusCode) -> String {
    match status {
        // 4xx Client Error
        StatusCode::BAD_REQUEST => "Bad Request",
        StatusCode::UNAUTHORIZED => "Unauthorized",
        StatusCode::PAYMENT_REQUIRED => "Payment Required",
        StatusCode::FORBIDDEN => "Forbidden",
        StatusCode::NOT_FOUND => "Resource not found",
        StatusCode::METHOD_NOT_ALLOWED => "Method Not Allowed",
        StatusCode::NOT_ACCEPTABLE => "Not Acceptable",
        StatusCode::PROXY_AUTHENTICATION_REQUIRED => "Proxy Authentication Required",
        StatusCode::REQUEST_TIMEOUT => "Request Timeout",
        StatusCode::CONFLICT => "Conflict",
        StatusCode::GONE => "Gone",
        StatusCode::LENGTH_REQUIRED => "Length Required",
        StatusCode::PRECONDITION_FAILED => "Precondition Failed",
        StatusCode::PAYLOAD_TOO_LARGE => "Payload Too Large",
        StatusCode::URI_TOO_LONG => "URI Too Long",
        StatusCode::UNSUPPORTED_MEDIA_TYPE => "Unsupported Media Type",
        StatusCode::RANGE_NOT_SATISFIABLE => "Range Not Satisfiable",
        StatusCode::EXPECTATION_FAILED => "Expectation Failed",
        StatusCode::UNPROCESSABLE_ENTITY => "Unprocessable Entity",
        StatusCode::LOCKED => "Locked",
        StatusCode::FAILED_DEPENDENCY => "Failed Dependency",
        StatusCode::TOO_EARLY => "Too Early",
        StatusCode::UPGRADE_REQUIRED => "Upgrade Required",
        StatusCode::PRECONDITION_REQUIRED => "Precondition Required",
        StatusCode::TOO_MANY_REQUESTS => "Too Many Requests",
        StatusCode::REQUEST_HEADER_FIELDS_TOO_LARGE => "Request Header Fields Too Large",

        // 5xx Server Error
        StatusCode::INTERNAL_SERVER_ERROR => "Internal server error",
        StatusCode::NOT_IMPLEMENTED => "Not Implemented",
        StatusCode::BAD_GATEWAY => "Bad Gateway",
        StatusCode::SERVICE_UNAVAILABLE => "Service Unavailable",
        StatusCode::GATEWAY_TIMEOUT => "Gateway Timeout",
        StatusCode::HTTP_VERSION_NOT_SUPPORTED => "HTTP Version Not Supported",
        StatusCode::VARIANT_ALSO_NEGOTIATES => "Variant Also Negotiates",
        StatusCode::INSUFFICIENT_STORAGE => "Insufficient Storage",
        StatusCode::LOOP_DETECTED => "Loop Detected",
        StatusCode::NOT_EXTENDED => "Not Extended",
        StatusCode::NETWORK_AUTHENTICATION_REQUIRED => "Network Authentication Required",
        _ => "An error occurred",
    }
    .to_string()
}
