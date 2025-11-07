use crate::features::service::controller::job_controller::JobController;
use crate::features::service::controller::service_controller::ServiceController;
use crate::middleware::auth::auth;
use crate::middleware::is_admin::is_admin;
use crate::utility::state::app_state;
use axum::routing::{get, post, put};
use axum::{Router, middleware};

pub fn service_route() -> (&'static str, Router) {
    let state = app_state();

    let middleware_auth = middleware::from_fn_with_state(state.clone(), auth);
    let middleware_is_admin = middleware::from_fn_with_state(state.clone(), is_admin);
    let admin_router = Router::new()
        .route(
            "/",
            get(ServiceController::list).post(ServiceController::create),
        )
        .route(
            "/{service_id}",
            put(ServiceController::update).delete(ServiceController::delete),
        )
        .route("/{service_id}/chat", post(ServiceController::chat))
        .route_layer(middleware_is_admin);

    (
        "api/v1/services",
        Router::new()
            .merge(admin_router)
            .route_layer(middleware_auth),
    )
}

pub fn jobs_route() -> (&'static str, Router) {
    let state = app_state();

    let middleware_auth = middleware::from_fn_with_state(state.clone(), auth);

    (
        "api/v1/jobs",
        Router::new()
            .route("/{job_id}", get(JobController::show))
            .route_layer(middleware_auth),
    )
}
