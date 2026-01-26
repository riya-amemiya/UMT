use base64::{Engine, engine::general_purpose::STANDARD};

/// Convert string to Base64
///
/// # Arguments
/// * `s` - String to convert to Base64
///
/// # Returns
/// Base64 encoded string
///
/// # Example
/// ```
/// use umt_rust::string::umt_to_base64;
/// assert_eq!(umt_to_base64("Hello World"), "SGVsbG8gV29ybGQ=");
/// ```
#[inline]
pub fn umt_to_base64(s: &str) -> String {
    STANDARD.encode(s.as_bytes())
}
