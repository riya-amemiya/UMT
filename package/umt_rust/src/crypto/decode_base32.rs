/// Error type for Base32 decoding failures.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Base32Error {
    pub message: String,
}

impl std::fmt::Display for Base32Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.message)
    }
}

impl std::error::Error for Base32Error {}

/// Decodes a Base32 string to bytes.
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
/// * `Ok(Vec<u8>)` - The decoded bytes.
/// * `Err(Base32Error)` - If the input contains invalid Base32 characters.
///
/// # Examples
///
/// ```
/// use umt_rust::crypto::umt_decode_base32;
///
/// let result = umt_decode_base32("JBSWY3DP").unwrap();
/// assert_eq!(String::from_utf8(result).unwrap(), "Hello");
/// ```
pub fn umt_decode_base32(input: &str) -> Result<Vec<u8>, Base32Error> {
    const ALPHABET: &str = "ABCDEFGHIJKLMNOPQRSTUVWXYZ234567";

    // Remove padding
    let cleaned_input: String = input.chars().filter(|&c| c != '=').collect();

    if cleaned_input.is_empty() {
        return Ok(Vec::new());
    }

    let mut result: Vec<u8> = Vec::new();
    let mut buffer: u32 = 0;
    let mut buffer_length: u32 = 0;

    for char in cleaned_input.chars() {
        let value = match ALPHABET.find(char) {
            Some(v) => v as u32,
            None => {
                return Err(Base32Error {
                    message: format!("Invalid base32 character: {}", char),
                });
            }
        };

        buffer = (buffer << 5) | value;
        buffer_length += 5;

        if buffer_length >= 8 {
            buffer_length -= 8;
            result.push(((buffer >> buffer_length) & 0xff) as u8);
        }
    }

    Ok(result)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_decode_simple_string() {
        let result = umt_decode_base32("JBSWY3DP").unwrap();
        assert_eq!(String::from_utf8(result).unwrap(), "Hello");
    }

    #[test]
    fn test_decode_empty_string() {
        let result = umt_decode_base32("").unwrap();
        assert!(result.is_empty());
    }

    #[test]
    fn test_decode_strings_with_padding() {
        assert_eq!(
            String::from_utf8(umt_decode_base32("MY======").unwrap()).unwrap(),
            "f"
        );
        assert_eq!(
            String::from_utf8(umt_decode_base32("MZXQ====").unwrap()).unwrap(),
            "fo"
        );
        assert_eq!(
            String::from_utf8(umt_decode_base32("MZXW6===").unwrap()).unwrap(),
            "foo"
        );
        assert_eq!(
            String::from_utf8(umt_decode_base32("MZXW6YQ=").unwrap()).unwrap(),
            "foob"
        );
        assert_eq!(
            String::from_utf8(umt_decode_base32("MZXW6YTB").unwrap()).unwrap(),
            "fooba"
        );
        assert_eq!(
            String::from_utf8(umt_decode_base32("MZXW6YTBOI======").unwrap()).unwrap(),
            "foobar"
        );
    }

    #[test]
    fn test_invalid_character_at_sign() {
        let result = umt_decode_base32("JBSWY3D@");
        assert!(result.is_err());
        assert_eq!(result.unwrap_err().message, "Invalid base32 character: @");
    }

    #[test]
    fn test_invalid_character_one() {
        let result = umt_decode_base32("JBSWY3D1");
        assert!(result.is_err());
        assert_eq!(result.unwrap_err().message, "Invalid base32 character: 1");
    }

    #[test]
    fn test_invalid_character_zero() {
        let result = umt_decode_base32("JBSWY3D0");
        assert!(result.is_err());
        assert_eq!(result.unwrap_err().message, "Invalid base32 character: 0");
    }

    #[test]
    fn test_decode_longer_text() {
        let encoded = "KRUGKIDROVUWG2ZAMJZG653OEBTG66BANJ2W24DTEBXXMZLSEB2GQZJANRQXU6JAMRXWO===";
        let result = String::from_utf8(umt_decode_base32(encoded).unwrap()).unwrap();
        assert_eq!(result, "The quick brown fox jumps over the lazy dog");
    }

    #[test]
    fn test_decode_without_padding() {
        assert_eq!(
            String::from_utf8(umt_decode_base32("JBSWY3DP").unwrap()).unwrap(),
            "Hello"
        );
        assert_eq!(
            String::from_utf8(umt_decode_base32("MZXW6YTB").unwrap()).unwrap(),
            "fooba"
        );
    }
}
