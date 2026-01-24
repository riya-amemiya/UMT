/// Truncate a string to a specified length
///
/// # Arguments
///
/// * `string_` - The string to truncate
/// * `length` - The maximum length
/// * `suffix` - The suffix to add when truncating (default: "...")
///
/// # Returns
///
/// The truncated string
///
/// # Errors
///
/// Returns an error if the length is negative (not applicable in Rust as usize is unsigned)
///
/// # Examples
///
/// ```
/// use umt_rust::string::umt_truncate;
///
/// assert_eq!(umt_truncate("Hello World", 5, "...").unwrap(), "Hello...");
/// assert_eq!(umt_truncate("Hello World", 5, "~").unwrap(), "Hello~");
/// assert_eq!(umt_truncate("Hello", 10, "...").unwrap(), "Hello");
/// ```
#[inline]
pub fn umt_truncate(string_: &str, length: usize, suffix: &str) -> Result<String, String> {
    let char_count = string_.chars().count();

    if char_count <= length {
        return Ok(string_.to_string());
    }

    let truncated: String = string_.chars().take(length).collect();
    Ok(format!("{}{}", truncated, suffix))
}

/// Truncate a string to a specified length (panics on negative length - kept for API compatibility)
///
/// # Arguments
///
/// * `string_` - The string to truncate
/// * `length` - The maximum length (must be non-negative)
/// * `suffix` - The suffix to add when truncating
///
/// # Panics
///
/// This function uses usize so cannot receive negative values
#[inline]
pub fn umt_truncate_checked(string_: &str, length: i64, suffix: &str) -> Result<String, String> {
    if length < 0 {
        return Err("Length must be non-negative".to_string());
    }
    umt_truncate(string_, length as usize, suffix)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_jsdoc_examples() {
        assert_eq!(umt_truncate("Hello World", 5, "...").unwrap(), "Hello...");
        assert_eq!(umt_truncate("Hello World", 5, "~").unwrap(), "Hello~");
        assert_eq!(umt_truncate("Hello", 10, "...").unwrap(), "Hello");
    }

    #[test]
    fn test_no_truncate_if_shorter_or_equal() {
        assert_eq!(umt_truncate("Hi", 5, "...").unwrap(), "Hi");
        assert_eq!(umt_truncate("Hello", 5, "...").unwrap(), "Hello");
    }

    #[test]
    fn test_empty_suffix() {
        assert_eq!(umt_truncate("Hello World", 5, "").unwrap(), "Hello");
    }

    #[test]
    fn test_zero_length() {
        assert_eq!(umt_truncate("Hello", 0, "").unwrap(), "");
        assert_eq!(umt_truncate("Hello", 0, "...").unwrap(), "...");
    }

    #[test]
    fn test_negative_length() {
        assert!(umt_truncate_checked("Hello", -1, "...").is_err());
    }

    #[test]
    fn test_suffix_longer_than_target() {
        assert_eq!(umt_truncate("Hello World", 2, "...").unwrap(), "He...");
        assert_eq!(umt_truncate("Hello World", 1, "...").unwrap(), "H...");
    }

    #[test]
    fn test_empty_string() {
        assert_eq!(umt_truncate("", 5, "...").unwrap(), "");
    }
}
