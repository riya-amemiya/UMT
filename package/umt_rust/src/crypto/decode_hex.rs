/// Error type for hexadecimal decoding failures.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct HexError {
    pub message: String,
}

impl std::fmt::Display for HexError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.message)
    }
}

impl std::error::Error for HexError {}

/// Decodes a hexadecimal string to bytes.
///
/// Both lowercase and uppercase digits are accepted.
///
/// # Arguments
///
/// * `input` - A hexadecimal string with two characters per byte.
///
/// # Returns
///
/// * `Ok(Vec<u8>)` - The decoded bytes.
/// * `Err(HexError)` - If the input has an odd length or contains a non-hex character.
///
/// # Examples
///
/// ```
/// use umt_rust::crypto::umt_decode_hex;
///
/// assert_eq!(umt_decode_hex("48656c6c6f").unwrap(), b"Hello");
/// assert_eq!(umt_decode_hex("00FF").unwrap(), vec![0, 255]);
/// assert!(umt_decode_hex("abc").is_err());
/// ```
pub fn umt_decode_hex(input: &str) -> Result<Vec<u8>, HexError> {
    let invalid = || HexError {
        message: "Invalid hex string".to_string(),
    };

    if !input.len().is_multiple_of(2) {
        return Err(invalid());
    }

    input
        .as_bytes()
        .chunks_exact(2)
        .map(|pair| {
            let high = char::from(pair[0]).to_digit(16).ok_or_else(invalid)?;
            let low = char::from(pair[1]).to_digit(16).ok_or_else(invalid)?;
            Ok(((high << 4) | low) as u8)
        })
        .collect()
}
