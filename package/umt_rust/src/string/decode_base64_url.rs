use base64::{
    Engine, alphabet,
    engine::{DecodePaddingMode, GeneralPurpose, GeneralPurposeConfig},
};

const URL_SAFE_FORGIVING: GeneralPurpose = GeneralPurpose::new(
    &alphabet::URL_SAFE,
    GeneralPurposeConfig::new()
        .with_encode_padding(false)
        .with_decode_padding_mode(DecodePaddingMode::RequireNone)
        .with_decode_allow_trailing_bits(true),
);

/// Converts URL-safe Base64 (RFC 4648 section 5) to string
///
/// Padding is optional. Missing `=` characters are added up to a multiple
/// of four first, so partial padding such as `"Zg="` is accepted.
///
/// # Arguments
/// * `base64_url_string` - URL-safe Base64 encoded string, with or without `=` padding
///
/// # Returns
/// * `Ok(String)` - Decoded string
/// * `Err("Invalid Base64 string")` - If the input is not valid URL-safe Base64
/// * `Err("Invalid UTF-8 sequence")` - If the decoded bytes are not valid UTF-8
///
/// # Example
/// ```
/// use umt_rust::string::umt_decode_base64_url;
/// assert_eq!(umt_decode_base64_url("SGVsbG8gV29ybGQ").unwrap(), "Hello World");
/// assert_eq!(umt_decode_base64_url("SGVsbG8gV29ybGQ=").unwrap(), "Hello World");
/// ```
pub fn umt_decode_base64_url(base64_url_string: &str) -> Result<String, String> {
    let data = base64_url_string.trim_end_matches('=');
    let padding = base64_url_string.len() - data.len();
    if padding + (4 - base64_url_string.len() % 4) % 4 > 2 {
        return Err("Invalid Base64 string".to_string());
    }

    URL_SAFE_FORGIVING
        .decode(data)
        .map_err(|_| "Invalid Base64 string".to_string())
        .and_then(|bytes| {
            String::from_utf8(bytes).map_err(|_| "Invalid UTF-8 sequence".to_string())
        })
}
