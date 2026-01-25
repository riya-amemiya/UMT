use base64::{Engine, engine::general_purpose::STANDARD};

/// Converts Base64 to string
///
/// # Arguments
/// * `base64_string` - Base64 encoded string
///
/// # Returns
/// Result containing decoded string or error
///
/// # Example
/// ```
/// use umt_rust::string::umt_from_base64;
/// assert_eq!(umt_from_base64("SGVsbG8gV29ybGQ=").unwrap(), "Hello World");
/// ```
#[inline]
pub fn umt_from_base64(base64_string: &str) -> Result<String, String> {
    if base64_string.is_empty() {
        return Ok(String::new());
    }

    STANDARD
        .decode(base64_string)
        .map_err(|_| "Invalid Base64 string".to_string())
        .and_then(|bytes| {
            String::from_utf8(bytes).map_err(|_| "Invalid UTF-8 sequence".to_string())
        })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_from_base64_basic() {
        assert_eq!(umt_from_base64("SGVsbG8gV29ybGQ=").unwrap(), "Hello World");
    }

    #[test]
    fn test_from_base64_empty() {
        assert_eq!(umt_from_base64("").unwrap(), "");
    }

    #[test]
    fn test_from_base64_invalid() {
        assert!(umt_from_base64("!!!invalid!!!").is_err());
    }
}
