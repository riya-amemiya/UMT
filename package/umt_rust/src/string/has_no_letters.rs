/// Checks if the string contains no letters (contains only emojis, numbers, or special characters)
///
/// # Arguments
/// * `text` - The string to check
///
/// # Returns
/// True if the string has no letters, false otherwise
///
/// # Example
/// ```
/// use umt_rust::string::umt_has_no_letters;
/// assert!(umt_has_no_letters("123"));
/// assert!(umt_has_no_letters("!@#$%"));
/// assert!(!umt_has_no_letters("abc123"));
/// ```
#[inline]
pub fn umt_has_no_letters(text: &str) -> bool {
    !text.chars().any(|c| c.is_alphabetic())
}
