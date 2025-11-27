pub struct JobExecutionContext {
    pub job_id: i64,
    pub provider: String,
    pub model: String,
    pub key: String,
    pub message: String,
}

impl JobExecutionContext {
    pub fn new(job_id: i64, provider: String, model: String, key: String, message: String) -> Self {
        Self {
            job_id,
            provider,
            model,
            key,
            message,
        }
    }
}
