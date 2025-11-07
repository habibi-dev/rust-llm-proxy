use crate::features::service::model::service;
use blake3::Hasher;

pub fn generate(message: &str, service: &service::Model) -> String {
    let mut hasher = Hasher::new();
    hasher.update(service.provider.as_bytes());
    hasher.update(service.key.as_bytes());
    hasher.update(message.as_bytes());
    hasher.finalize().to_hex().to_string()
}
