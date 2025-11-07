use crate::core::app_error::{handle_normalize_error, handle_tower_error};
use crate::core::response::json_error;
use crate::core::state::AppState;
use axum::error_handling::HandleErrorLayer;
use axum::extract::Path;
use axum::http::{StatusCode, header};
use axum::response::{IntoResponse, Response};
use axum::routing::get;
use axum::{Router as AxumRouter, body::Body, middleware};
use rust_embed::RustEmbed;
use std::time::Duration;
use tower::ServiceBuilder;
use tower::timeout::TimeoutLayer;
use tower_http::trace::TraceLayer;

#[derive(RustEmbed)]
#[folder = "src/assets/"]
struct Assets;

pub struct Router;
impl Router {
    pub fn routes(app_state: AppState, route_lists: Vec<(&str, axum::Router)>) -> AxumRouter {
        let mut route_list = AxumRouter::new();

        for (prefix_raw, routes) in route_lists {
            let prefix = Self::normalize_prefix(prefix_raw);
            // Use merge at root, nest otherwise
            route_list = if prefix.is_empty() || prefix == "/" {
                route_list.merge(routes)
            } else {
                route_list.nest(&prefix, routes)
            };
        }

        AxumRouter::new()
            .with_state(app_state.clone())
            .merge(route_list)
            .route("/assets/{*path}", get(Self::assets))
            .fallback(Self::not_found)
            .layer(
                ServiceBuilder::new()
                    .layer(HandleErrorLayer::new(handle_tower_error))
                    .layer(TimeoutLayer::new(Duration::from_secs(30))) // inner
                    .layer(TraceLayer::new_for_http())
                    .into_inner(),
            )
            .route_layer(middleware::from_fn(handle_normalize_error))
    }

    pub async fn assets(Path(path): Path<String>) -> impl IntoResponse {
        let path = path.trim_start_matches('/');

        match Assets::get(path) {
            Some(content) => {
                let mime = mime_guess::from_path(path).first_or_octet_stream();

                Response::builder()
                    .status(StatusCode::OK)
                    .header(header::CONTENT_TYPE, mime.as_ref())
                    .body(Body::from(content.data.to_vec()))
                    .unwrap()
            }
            None => json_error(StatusCode::NOT_FOUND, "Asset not found").into_response(),
        }
    }

    async fn not_found() -> impl IntoResponse {
        json_error(StatusCode::NOT_FOUND, "Route not found").into_response()
    }

    fn normalize_prefix(raw: &str) -> String {
        let s = raw.trim();
        if s.is_empty() || s == "/" {
            return "/".to_string();
        }
        let s = s.trim_matches('/');
        format!("/{}", s)
    }
}
