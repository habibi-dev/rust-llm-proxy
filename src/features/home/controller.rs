use askama::Template;
use axum::http::StatusCode;
use axum::response::{Html, IntoResponse, Response};

#[derive(Template)]
#[template(path = "index.html")]
struct IndexTemplate {
    version: String,
}

pub struct HomeController;
impl HomeController {
    pub async fn index() -> impl IntoResponse {
        const VERSION: &str = env!("CARGO_PKG_VERSION");

        IndexTemplate {
            version: VERSION.into(),
        }
    }
}

impl IntoResponse for IndexTemplate {
    fn into_response(self) -> Response {
        match self.render() {
            Ok(html) => Html(html).into_response(),
            Err(_) => (
                StatusCode::INTERNAL_SERVER_ERROR,
                "Failed to render template",
            )
                .into_response(),
        }
    }
}
