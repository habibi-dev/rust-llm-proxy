use serde_json::{Value as Json, json};

pub struct JobPayloadBuilder;

impl JobPayloadBuilder {
    pub fn build_input(message: &str) -> Json {
        json!({ "message": message })
    }

    pub fn build_success_output(result: &str) -> Json {
        json!({ "response": result })
    }

    pub fn build_error_output(error: &str) -> Json {
        json!({ "error": error })
    }
}
