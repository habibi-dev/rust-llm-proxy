use serde::Deserialize;
use validator::Validate;

#[derive(Debug, Deserialize, Validate)]
pub struct ChatForm {
    pub message: String,
    pub system_message: Option<String>,
}
