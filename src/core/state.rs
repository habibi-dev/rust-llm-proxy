use crate::core::config::Config;
use sea_orm::DatabaseConnection;
use std::sync::OnceLock;
use std::time::Instant;

pub static APP_STATE: OnceLock<AppState> = OnceLock::new();

#[derive(Clone)]
pub struct AppState {
    pub _db: DatabaseConnection,
    pub config: Config,
    pub uptime: Instant,
}

pub struct State;

impl State {
    pub fn init(db: DatabaseConnection, config: Config) {
        APP_STATE
            .set(AppState {
                _db: db,
                config,
                uptime: Instant::now(),
            })
            .ok();
    }
}
