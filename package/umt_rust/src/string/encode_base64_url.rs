use base64::{Engine, engine::general_purpose::URL_SAFE_NO_PAD};

/// Convert string to URL-safe Base64 (RFC 4648 section 5) without padding
///
/// # Arguments
/// * `s` - String to convert to URL-safe Base64
///
/// # Returns
/// URL-safe Base64 encoded string using `-` and `_` and no `=` padding
///
/// # Example
/// ```
/// use umt_rust::string::umt_encode_base64_url;
/// assert_eq!(umt_encode_base64_url("Hello World"), "SGVsbG8gV29ybGQ");
/// assert_eq!(umt_encode_base64_url("\u{00FF}\u{00FE}"), "w7_Dvg");
/// ```
#[inline]
pub fn umt_encode_base64_url(s: &str) -> String {
    URL_SAFE_NO_PAD.encode(s.as_bytes())
}
