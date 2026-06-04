/// Ensures a string starts with the given prefix, prepending it only when it is
/// not already present.
///
/// # Arguments
/// * `s` - The input string
/// * `prefix` - The prefix to ensure
///
/// # Returns
/// A string guaranteed to start with the prefix
///
/// # Examples
/// ```
/// use umt_rust::string::umt_ensure_prefix;
/// assert_eq!(umt_ensure_prefix("example.com", "https://"), "https://example.com");
/// assert_eq!(umt_ensure_prefix("https://example.com", "https://"), "https://example.com");
/// ```
#[inline]
pub fn umt_ensure_prefix(s: &str, prefix: &str) -> String {
    if s.starts_with(prefix) {
        s.to_string()
    } else {
        format!("{prefix}{s}")
    }
}
