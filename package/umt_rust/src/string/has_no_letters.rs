/// Checks if the string contains no letters (contains only emojis, numbers, or special characters)
///
/// # Arguments
///
/// * `text` - The string to check
///
/// # Returns
///
/// True if the string has no letters, false otherwise
///
/// # Examples
///
/// ```
/// use umt_rust::string::umt_has_no_letters;
///
/// assert!(umt_has_no_letters("123"));
/// assert!(umt_has_no_letters("\u{1F31F}123#"));
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
    fn test_strings_without_letters() {
        assert!(umt_has_no_letters("")); // Empty string
        assert!(umt_has_no_letters("12345")); // Numbers only
        assert!(umt_has_no_letters("!@#$%")); // Special characters
        assert!(umt_has_no_letters("     ")); // Whitespace only
    }

    #[test]
    fn test_strings_with_emojis_and_symbols() {
        assert!(umt_has_no_letters("\u{1F389}")); // Single emoji
        assert!(umt_has_no_letters("\u{1F600}\u{1F603}\u{1F604}")); // Multiple emojis
        // Decorative symbols
        assert!(umt_has_no_letters("\u{2765}\u{FF65}\u{2022}\u{2744}"));
        // Full-width question marks
        assert!(umt_has_no_letters("\u{FF1F}\u{FF1F}\u{FF1F}\u{FF1F}\u{FF1F}"));
        assert!(umt_has_no_letters("\n      \u{1F388}\n      \u{1F38A}\n      \u{1F389}\n    ")); // Multiline emojis
    }

    #[test]
    fn test_strings_with_letters() {
        assert!(!umt_has_no_letters("hello")); // English text
        assert!(!umt_has_no_letters("test 123")); // Mixed with numbers
        assert!(!umt_has_no_letters("\u{3053}\u{3093}\u{306B}\u{3061}\u{306F}")); // Japanese text
        assert!(!umt_has_no_letters("Caf\u{E9}")); // Accented characters
    }

    #[test]
    fn test_strings_with_mixed_content() {
        assert!(!umt_has_no_letters("hello \u{1F44B}")); // Text with emoji
        assert!(!umt_has_no_letters("\u{1F389} party")); // Emoji with text
        assert!(!umt_has_no_letters("\n      Hello\n      World\n      \u{1F604}\n      123\n    ")); // Multiline mixed content
    }
}
