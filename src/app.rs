use crate::core::config::Config;
use crate::core::http::start_http;
use crate::core::state::{APP_STATE, State};
use crate::routes::Routes;
use anyhow::Context;

pub async fn app() -> anyhow::Result<()> {
    // Load configuration
    let config = Config::load();

    // Setup database connection
    let db = Config::setup_database().await?;

    // Initialize application state
    State::init(db, config.clone());
    let state = APP_STATE
        .get()
        .cloned()
        .context("Application state not initialized")?;

    // Start background jobs

    // Setup routes and middleware
    let routes = Routes::generate(state);

    // Start the HTTP server
    start_http(routes, &config)
        .await
        .map_err(|e| anyhow::anyhow!("Failed to start HTTP server: {}", e))?;

    Ok(())
}
