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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_has_no_letters_numbers() {
        assert!(umt_has_no_letters("123"));
    }

    #[test]
    fn test_has_no_letters_special() {
        assert!(umt_has_no_letters("!@#$%"));
    }

    #[test]
    fn test_has_no_letters_with_letters() {
        assert!(!umt_has_no_letters("abc123"));
    }

    #[test]
    fn test_has_no_letters_empty() {
        assert!(umt_has_no_letters(""));
    }

    #[test]
    fn test_has_no_letters_only_letters() {
        assert!(!umt_has_no_letters("abc"));
    }
}
