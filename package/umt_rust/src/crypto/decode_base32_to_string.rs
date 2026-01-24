use super::decode_base32::{umt_decode_base32, Base32Error};

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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_decode_simple_string() {
        assert_eq!(umt_decode_base32_to_string("JBSWY3DP").unwrap(), "Hello");
    }

    #[test]
    fn test_decode_empty_string() {
        assert_eq!(umt_decode_base32_to_string("").unwrap(), "");
    }

    #[test]
    fn test_decode_strings_with_padding() {
        assert_eq!(umt_decode_base32_to_string("MY======").unwrap(), "f");
        assert_eq!(umt_decode_base32_to_string("MZXQ====").unwrap(), "fo");
        assert_eq!(umt_decode_base32_to_string("MZXW6===").unwrap(), "foo");
        assert_eq!(umt_decode_base32_to_string("MZXW6YQ=").unwrap(), "foob");
        assert_eq!(umt_decode_base32_to_string("MZXW6YTB").unwrap(), "fooba");
        assert_eq!(
            umt_decode_base32_to_string("MZXW6YTBOI======").unwrap(),
            "foobar"
        );
    }

    #[test]
    fn test_decode_special_characters() {
        assert_eq!(
            umt_decode_base32_to_string("4OAZHY4CSPRYDK7DQGQ6HANP").unwrap(),
            "こんにちは"
        );
    }

    #[test]
    fn test_invalid_character() {
        let result = umt_decode_base32_to_string("JBSWY3D@");
        assert!(result.is_err());
        assert_eq!(
            result.unwrap_err().message,
            "Invalid base32 character: @"
        );
    }

    #[test]
    fn test_decode_longer_text() {
        let encoded =
            "KRUGKIDROVUWG2ZAMJZG653OEBTG66BANJ2W24DTEBXXMZLSEB2GQZJANRQXU6JAMRXWO===";
        assert_eq!(
            umt_decode_base32_to_string(encoded).unwrap(),
            "The quick brown fox jumps over the lazy dog"
        );
    }
}
