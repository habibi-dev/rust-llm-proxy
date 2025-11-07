use chrono::Utc;

pub fn generate(hash: &str) -> String {
    let now = Utc::now();
    let timestamp = now
        .timestamp_nanos_opt()
        .unwrap_or_else(|| now.timestamp_micros() * 1_000);
    format!("{hash}-{timestamp}")
}
