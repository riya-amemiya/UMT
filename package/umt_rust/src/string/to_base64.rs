use base64::{engine::general_purpose::STANDARD, Engine as _};

/// Convert string to Base64
///
/// # Arguments
///
/// * `char_` - String to convert to Base64
///
/// # Returns
///
/// Base64 encoded string
///
/// # Examples
///
/// ```
/// use umt_rust::string::umt_to_base64;
///
/// assert_eq!(umt_to_base64("test"), "dGVzdA==");
/// assert_eq!(umt_to_base64(""), "");
/// ```
#[inline]
pub fn umt_to_base64(char_: &str) -> String {
    STANDARD.encode(char_.as_bytes())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_convert_string_to_base64() {
        assert_eq!(umt_to_base64("test"), "dGVzdA==");
    }

    #[test]
    fn test_empty_string() {
        assert_eq!(umt_to_base64(""), "");
    }

    #[test]
    fn test_special_characters() {
        assert_eq!(umt_to_base64("@#%"), "QCMl");
    }

    #[test]
    fn test_japanese_characters() {
        assert_eq!(
            umt_to_base64("\u{3042}\u{3044}\u{3046}\u{3048}\u{304A}"),
            "44GC44GE44GG44GI44GK"
        );
    }

    #[test]
    fn test_long_string() {
        assert_eq!(
            umt_to_base64("This is a long string to test the Base64 conversion"),
            "VGhpcyBpcyBhIGxvbmcgc3RyaW5nIHRvIHRlc3QgdGhlIEJhc2U2NCBjb252ZXJzaW9u"
        );
    }
}
