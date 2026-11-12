/// Truncate a string to a specified length
///
/// # Arguments
/// * `s` - The string to truncate
/// * `length` - The maximum length
/// * `suffix` - The suffix to add when truncating (default: "...")
///
/// # Returns
/// The truncated string
///
/// # Example
/// ```
/// use umt_rust::string::umt_truncate;
/// assert_eq!(umt_truncate("Hello World", 5, "..."), "Hello...");
/// assert_eq!(umt_truncate("Hello World", 5, "~"), "Hello~");
/// assert_eq!(umt_truncate("Hello", 10, "..."), "Hello");
/// ```
#[inline]
pub fn umt_truncate(s: &str, length: usize, suffix: &str) -> String {
    match s.char_indices().nth(length) {
        Some((cut, _)) => {
            let mut result = String::with_capacity(cut + suffix.len());
            result.push_str(&s[..cut]);
            result.push_str(suffix);
            result
        }
        None => s.to_string(),
    }
}

/// Truncate a string to a specified length with default suffix "..."
#[inline]
pub fn umt_truncate_default(s: &str, length: usize) -> String {
    umt_truncate(s, length, "...")
}
