use regex::Regex;

/// Collapses consecutive whitespace characters into a single space and trims the
/// result.
///
/// Unlike a function that removes all whitespace, this keeps words separated by
/// single spaces.
///
/// # Arguments
/// * `s` - The string to normalize
///
/// # Returns
/// The string with normalized whitespace
///
/// # Examples
/// ```
/// use umt_rust::string::umt_normalize_whitespace;
/// assert_eq!(umt_normalize_whitespace("  hello   world \t\n foo "), "hello world foo");
/// ```
#[inline]
pub fn umt_normalize_whitespace(s: &str) -> String {
    let whitespace = Regex::new(r"\s+").unwrap();
    whitespace.replace_all(s, " ").trim().to_string()
}
