// Represents an incoming chat prompt with optional system context to keep provider APIs aligned.
#[derive(Clone, Debug)]
pub struct ChatPrompt {
    pub user_message: String,
    pub system_message: Option<String>,
}

impl ChatPrompt {
    pub fn new(user_message: String, system_message: Option<String>) -> Self {
        Self {
            user_message,
            system_message,
        }
    }

    pub fn has_system_message(&self) -> bool {
        self.system_message
            .as_ref()
            .map(|value| !value.trim().is_empty())
            .unwrap_or(false)
    }
}
