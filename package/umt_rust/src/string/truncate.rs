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
    let chars: Vec<char> = s.chars().collect();
    if chars.len() <= length {
        return s.to_string();
    }

    let truncated: String = chars.into_iter().take(length).collect();
    format!("{}{}", truncated, suffix)
}

/// Truncate a string to a specified length with default suffix "..."
#[inline]
pub fn umt_truncate_default(s: &str, length: usize) -> String {
    umt_truncate(s, length, "...")
}
