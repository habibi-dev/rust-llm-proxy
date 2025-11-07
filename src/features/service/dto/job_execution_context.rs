pub struct JobExecutionContext {
    pub job_id: i64,
    pub provider: String,
    pub key: String,
    pub message: String,
}

impl JobExecutionContext {
    pub fn new(job_id: i64, provider: String, key: String, message: String) -> Self {
        Self {
            job_id,
            provider,
            key,
            message,
        }
    }
}
