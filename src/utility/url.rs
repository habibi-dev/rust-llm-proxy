use crate::utility::state::app_state;

pub fn url(path: &str) -> String {
    let config = app_state().config.clone();
    let domain = config.final_domain;
    let https = config.https;

    let scheme = if https { "https" } else { "http" };

    let clean_path = path.trim_start_matches('/');
    format!("{scheme}://{domain}/{}", clean_path)
}
