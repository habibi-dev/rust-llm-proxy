use axum::extract::RawForm;
use serde::de::DeserializeOwned;

/// Utility to parse nested `application/x-www-form-urlencoded` payloads (e.g. `field[sub]=value`).
/// Uses `serde_qs` to support bracketed keys while keeping controller code minimal.
pub fn parse_nested_form<T>(raw_form: &RawForm) -> Result<T, String>
where
    T: DeserializeOwned,
{
    serde_qs::from_bytes(raw_form.0.as_ref())
        .map_err(|error| format!("Invalid form payload: {error}"))
}
