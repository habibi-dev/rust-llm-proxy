use serde::Deserialize;
use validator::Validate;

#[derive(Debug, Deserialize, Validate)]
pub struct UserForm {
    #[validate(length(min = 3, max = 32))]
    pub name: String,
    #[validate(required)]
    pub status: Option<bool>,
    #[validate(required)]
    pub is_admin: Option<bool>,
}
