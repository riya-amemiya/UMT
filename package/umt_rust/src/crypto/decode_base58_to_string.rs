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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_decode_simple_string() {
        assert_eq!(umt_decode_base58_to_string("9Ajdvzr").unwrap(), "Hello");
    }

    #[test]
    fn test_decode_empty_string() {
        assert_eq!(umt_decode_base58_to_string("").unwrap(), "");
    }

    #[test]
    fn test_decode_single_character_strings() {
        assert_eq!(umt_decode_base58_to_string("2g").unwrap(), "a");
        assert_eq!(umt_decode_base58_to_string("2h").unwrap(), "b");
        assert_eq!(umt_decode_base58_to_string("2i").unwrap(), "c");
    }

    #[test]
    fn test_decode_special_characters() {
        assert_eq!(
            umt_decode_base58_to_string("7NAasPYBzpyEe5hmwr1KL").unwrap(),
            "こんにちは"
        );
    }

    #[test]
    fn test_invalid_character_zero() {
        let result = umt_decode_base58_to_string("9Ajdvz0");
        assert!(result.is_err());
        assert_eq!(result.unwrap_err().message, "Invalid base58 character: 0");
    }

    #[test]
    fn test_invalid_character_uppercase_o() {
        let result = umt_decode_base58_to_string("9AjdvzO");
        assert!(result.is_err());
        assert_eq!(result.unwrap_err().message, "Invalid base58 character: O");
    }

    #[test]
    fn test_decode_longer_text() {
        let encoded = "7DdiPPYtxLjCD3wA1po2rvZHTDYjkZYiEtazrfiwJcwnKCizhGFhBGHeRdx";
        assert_eq!(
            umt_decode_base58_to_string(encoded).unwrap(),
            "The quick brown fox jumps over the lazy dog"
        );
    }
}
