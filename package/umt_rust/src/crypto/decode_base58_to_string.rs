use super::decode_base58::{Base58Error, umt_decode_base58};

/// Decodes a Base58 string to a UTF-8 string.
///
/// Uses the Bitcoin Base58 alphabet which excludes ambiguous characters
/// (0, O, I, l) to improve readability.
///
/// # Arguments
///
/// * `input` - A Base58 encoded string.
///
/// # Returns
///
/// * `Ok(String)` - The decoded UTF-8 string.
/// * `Err(Base58Error)` - If the input contains invalid Base58 characters or
///   if the decoded bytes are not valid UTF-8.
///
/// # Examples
///
/// ```
/// use umt_rust::crypto::umt_decode_base58_to_string;
///
/// assert_eq!(umt_decode_base58_to_string("9Ajdvzr").unwrap(), "Hello");
/// ```
pub fn umt_decode_base58_to_string(input: &str) -> Result<String, Base58Error> {
    let bytes = umt_decode_base58(input)?;
    String::from_utf8(bytes).map_err(|e| Base58Error {
        message: format!("Invalid UTF-8 sequence: {}", e),
    })
}
