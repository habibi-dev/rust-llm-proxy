use crate::core::state::{APP_STATE, AppState};

pub fn app_state() -> &'static AppState {
    APP_STATE.get().expect("App state not initialized")
}
