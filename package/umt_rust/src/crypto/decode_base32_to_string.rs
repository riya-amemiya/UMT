use super::decode_base32::{Base32Error, umt_decode_base32};

/// Decodes a Base32 string to a UTF-8 string.
///
/// Uses the standard RFC 4648 Base32 alphabet (A-Z, 2-7).
/// Padding characters ('=') are handled automatically.
///
/// # Arguments
///
/// * `input` - A Base32 encoded string.
///
/// # Returns
///
/// * `Ok(String)` - The decoded UTF-8 string.
/// * `Err(Base32Error)` - If the input contains invalid Base32 characters or
///   if the decoded bytes are not valid UTF-8.
///
/// # Examples
///
/// ```
/// use umt_rust::crypto::umt_decode_base32_to_string;
///
/// assert_eq!(umt_decode_base32_to_string("JBSWY3DP").unwrap(), "Hello");
/// ```
pub fn umt_decode_base32_to_string(input: &str) -> Result<String, Base32Error> {
    let bytes = umt_decode_base32(input)?;
    String::from_utf8(bytes).map_err(|e| Base32Error {
        message: format!("Invalid UTF-8 sequence: {}", e),
    })
}
