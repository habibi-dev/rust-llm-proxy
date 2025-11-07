use crate::features::users::controller::api_key_controller::ApiKeyController;
use crate::features::users::controller::user_controller::UserController;
use crate::middleware::auth::auth;
use crate::middleware::is_admin::is_admin;
use crate::utility::state::app_state;
use axum::routing::{get, put};
use axum::{Router, middleware};

pub fn users_route() -> (&'static str, Router) {
    let state = app_state();

    let middleware_auth = middleware::from_fn_with_state(state.clone(), auth);
    let middleware_is_admin = middleware::from_fn_with_state(state.clone(), is_admin);

    let admin_router = Router::new()
        .route(
            "/",
            get(UserController::user_list).post(UserController::user_create),
        )
        .route(
            "/{user_id}",
            put(UserController::user_update).delete(UserController::user_delete),
        )
        .route_layer(middleware_is_admin);

    (
        "api/v1/users",
        Router::new()
            .merge(admin_router)
            .route("/me", get(UserController::me))
            .route_layer(middleware_auth),
    )
}

pub fn api_key_route() -> (&'static str, Router) {
    let state = app_state();

    let middleware_auth = middleware::from_fn_with_state(state.clone(), auth);
    let middleware_is_admin = middleware::from_fn_with_state(state.clone(), is_admin);

    let admin_router = Router::new()
        .route(
            "/",
            get(ApiKeyController::list).post(ApiKeyController::create),
        )
        .route(
            "/{api_key}",
            put(ApiKeyController::update).delete(ApiKeyController::delete),
        )
        .route_layer(middleware_is_admin);

    (
        "api/v1/api-keys",
        Router::new()
            .merge(admin_router)
            .route_layer(middleware_auth),
    )
}
