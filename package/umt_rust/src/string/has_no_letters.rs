use regex::Regex;
use std::sync::LazyLock;

static LETTER_RE: LazyLock<Regex> = LazyLock::new(|| Regex::new(r"\p{L}").unwrap());

/// Checks if the string contains no letters (contains only emojis, numbers, or special characters)
///
/// A letter is any character in the Unicode general category `L`.
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
    !LETTER_RE.is_match(text)
}
