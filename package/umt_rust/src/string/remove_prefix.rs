/// Removes the given prefix from the start of a string once, if present.
///
/// Unlike repeated trimming, this removes the exact prefix a single time. An
/// empty prefix leaves the string unchanged.
///
/// # Arguments
/// * `s` - The input string
/// * `prefix` - The prefix to remove
///
/// # Returns
/// The string without the leading prefix
///
/// # Examples
/// ```
/// use umt_rust::string::umt_remove_prefix;
/// assert_eq!(umt_remove_prefix("https://example.com", "https://"), "example.com");
/// assert_eq!(umt_remove_prefix("//path", "/"), "/path");
/// ```
#[inline]
pub fn umt_remove_prefix(s: &str, prefix: &str) -> String {
    if !prefix.is_empty() {
        if let Some(rest) = s.strip_prefix(prefix) {
            return rest.to_string();
        }
    }
    s.to_string()
}
