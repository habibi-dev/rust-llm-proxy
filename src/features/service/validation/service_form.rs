use serde::Deserialize;
use validator::Validate;

#[derive(Debug, Deserialize, Validate)]
pub struct ServiceForm {
    pub title: String,
    pub provider: String,
    pub key: Option<String>,
    pub model: Option<String>,
    pub status: Option<bool>,
}
