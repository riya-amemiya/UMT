/// Ensures a string ends with the given suffix, appending it only when it is
/// not already present.
///
/// # Arguments
/// * `s` - The input string
/// * `suffix` - The suffix to ensure
///
/// # Returns
/// A string guaranteed to end with the suffix
///
/// # Examples
/// ```
/// use umt_rust::string::umt_ensure_suffix;
/// assert_eq!(umt_ensure_suffix("file", ".txt"), "file.txt");
/// assert_eq!(umt_ensure_suffix("file.txt", ".txt"), "file.txt");
/// ```
#[inline]
pub fn umt_ensure_suffix(s: &str, suffix: &str) -> String {
    if s.ends_with(suffix) {
        s.to_string()
    } else {
        format!("{s}{suffix}")
    }
}
