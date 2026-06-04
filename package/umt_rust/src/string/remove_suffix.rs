/// Removes the given suffix from the end of a string once, if present.
///
/// Unlike repeated trimming, this removes the exact suffix a single time. An
/// empty suffix leaves the string unchanged.
///
/// # Arguments
/// * `s` - The input string
/// * `suffix` - The suffix to remove
///
/// # Returns
/// The string without the trailing suffix
///
/// # Examples
/// ```
/// use umt_rust::string::umt_remove_suffix;
/// assert_eq!(umt_remove_suffix("file.txt", ".txt"), "file");
/// assert_eq!(umt_remove_suffix("path//", "/"), "path/");
/// ```
#[inline]
pub fn umt_remove_suffix(s: &str, suffix: &str) -> String {
    if !suffix.is_empty() {
        if let Some(rest) = s.strip_suffix(suffix) {
            return rest.to_string();
        }
    }
    s.to_string()
}
