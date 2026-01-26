use super::trim_end_characters::umt_trim_end_characters;
use super::trim_start_characters::umt_trim_start_characters;

/// Removes specified characters from both ends of a string
///
/// # Arguments
/// * `s` - The string to trim
/// * `chars` - The set of characters to remove
///
/// # Returns
/// A new string with specified characters removed from both ends
///
/// # Example
/// ```
/// use umt_rust::string::umt_trim_characters;
/// assert_eq!(umt_trim_characters("!!!hello!!!", "!"), "hello");
/// assert_eq!(umt_trim_characters("---123---", "-"), "123");
/// ```
#[inline]
pub fn umt_trim_characters(s: &str, chars: &str) -> String {
    umt_trim_end_characters(&umt_trim_start_characters(s, chars), chars)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_trim_characters_both() {
        assert_eq!(umt_trim_characters("!!!hello!!!", "!"), "hello");
    }

    #[test]
    fn test_trim_characters_dashes() {
        assert_eq!(umt_trim_characters("---123---", "-"), "123");
    }

    #[test]
    fn test_trim_characters_no_match() {
        assert_eq!(umt_trim_characters("abc123", "xyz"), "abc123");
    }

    #[test]
    fn test_trim_characters_empty() {
        assert_eq!(umt_trim_characters("", "!"), "");
    }
}
