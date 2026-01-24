use base64::{engine::general_purpose::STANDARD, Engine};

/// Convert string to Base64
///
/// # Arguments
/// * `s` - String to convert to Base64
///
/// # Returns
/// Base64 encoded string
///
/// # Example
/// ```
/// use umt_rust::string::umt_to_base64;
/// assert_eq!(umt_to_base64("Hello World"), "SGVsbG8gV29ybGQ=");
/// ```
#[inline]
pub fn umt_to_base64(s: &str) -> String {
    STANDARD.encode(s.as_bytes())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_to_base64_basic() {
        assert_eq!(umt_to_base64("Hello World"), "SGVsbG8gV29ybGQ=");
    }

    #[test]
    fn test_to_base64_empty() {
        assert_eq!(umt_to_base64(""), "");
    }

    #[test]
    fn test_to_base64_unicode() {
        let encoded = umt_to_base64("Hello, World!");
        assert_eq!(encoded, "SGVsbG8sIFdvcmxkIQ==");
    }
}
