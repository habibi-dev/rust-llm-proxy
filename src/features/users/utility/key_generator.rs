use rand::Rng;
use rand::distr::Alphanumeric;

pub fn key_generator() -> String {
    rand::rng()
        .sample_iter(Alphanumeric)
        .take(128)
        .map(char::from)
        .collect()
}
