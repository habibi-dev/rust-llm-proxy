use crate::core::router::Router as MyRouter;
use crate::core::state::AppState;
use crate::features::home::controller::HomeController;
use crate::features::service::routes::{jobs_route, service_route};
use crate::features::users::routes::{api_key_route, users_route};
use axum::routing::get;
use axum::{Router as AxumRouter, Router};

pub(crate) struct Routes;
impl Routes {
    pub fn generate(app_state: AppState) -> AxumRouter {
        let routers_list: Vec<(&str, Router)> = vec![
            ("/", Router::new().route("/", get(HomeController::index))),
            users_route(),
            api_key_route(),
            service_route(),
            jobs_route(),
        ];

        MyRouter::routes(app_state, routers_list)
    }
}
