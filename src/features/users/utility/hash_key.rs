use blake3::Hasher;
use std::env;

pub fn hash_key(raw_key: &str) -> String {
    let hmac = env::var("HMAC_KEY").unwrap_or_default();
    let mut hasher = Hasher::new();
    hasher.update(hmac.as_bytes());
    hasher.update(raw_key.as_bytes());
    hasher.finalize().to_hex().to_string()
}
