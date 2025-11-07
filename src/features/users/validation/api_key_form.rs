use serde::Deserialize;
use validator::Validate;

#[derive(Debug, Deserialize, Validate)]
pub struct ApiKeyForm {
    pub user_id: Option<i64>,
    pub key: Option<String>,
    pub key_hash: Option<String>,
    pub status: Option<bool>,
}
