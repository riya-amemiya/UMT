use base64::{engine::general_purpose::STANDARD, Engine as _};

/// Converts Base64 to string
///
/// # Arguments
///
/// * `base64_string` - Base64 encoded string
///
/// # Returns
///
/// Decoded string from Base64
///
/// # Errors
///
/// Returns an error when input is not a valid Base64 string
///
/// # Examples
///
/// ```
/// use umt_rust::string::umt_from_base64;
///
/// assert_eq!(umt_from_base64("dGVzdA==").unwrap(), "test");
/// assert_eq!(umt_from_base64("").unwrap(), "");
/// ```
#[inline]
pub fn umt_from_base64(base64_string: &str) -> Result<String, String> {
    if base64_string.is_empty() {
        return Ok(String::new());
    }

    let decoded = STANDARD
        .decode(base64_string)
        .map_err(|_| "Invalid Base64 string".to_string())?;

    String::from_utf8(decoded).map_err(|_| "Invalid Base64 string".to_string())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::string::umt_to_base64;

    #[test]
    fn test_convert_base64_to_string() {
        assert_eq!(umt_from_base64("dGVzdA==").unwrap(), "test");
        assert_eq!(
            umt_from_base64(&umt_to_base64("test")).unwrap(),
            "test"
        );
    }

    #[test]
    fn test_empty_string() {
        assert_eq!(umt_from_base64("").unwrap(), "");
    }

    #[test]
    fn test_special_characters() {
        assert_eq!(umt_from_base64("QCMl").unwrap(), "@#%");
        assert_eq!(
            umt_from_base64(&umt_to_base64("@#%")).unwrap(),
            "@#%"
        );
    }

    #[test]
    fn test_japanese_characters() {
        assert_eq!(
            umt_from_base64("44GC44GE44GG44GI44GK").unwrap(),
            "\u{3042}\u{3044}\u{3046}\u{3048}\u{304A}"
        );
        assert_eq!(
            umt_from_base64(&umt_to_base64("\u{3042}\u{3044}\u{3046}\u{3048}\u{304A}")).unwrap(),
            "\u{3042}\u{3044}\u{3046}\u{3048}\u{304A}"
        );
    }

    #[test]
    fn test_emojis() {
        assert_eq!(
            umt_from_base64(&umt_to_base64("\u{1F30A}\u{1F30D}\u{1F30E}")).unwrap(),
            "\u{1F30A}\u{1F30D}\u{1F30E}"
        );
    }

    #[test]
    fn test_different_padding_patterns() {
        assert_eq!(umt_from_base64("YQ==").unwrap(), "a"); // 2 padding chars
        assert_eq!(umt_from_base64("YWE=").unwrap(), "aa"); // 1 padding char
        assert_eq!(umt_from_base64("YWFh").unwrap(), "aaa"); // no padding
    }

    #[test]
    fn test_invalid_base64() {
        assert!(umt_from_base64("abc@!#").is_err());
        assert!(umt_from_base64("=abc").is_err());
    }
}
