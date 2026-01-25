/// Removes specified characters from the start of a string
///
/// # Arguments
/// * `s` - The input string to trim
/// * `chars` - Characters to remove from the start
///
/// # Returns
/// A new string with specified characters removed from the start
///
/// # Example
/// ```
/// use umt_rust::string::umt_trim_start_characters;
/// assert_eq!(umt_trim_start_characters("!!!hello", "!"), "hello");
/// assert_eq!(umt_trim_start_characters("---123", "-"), "123");
/// assert_eq!(umt_trim_start_characters("abc123", "xyz"), "abc123");
/// ```
#[inline]
pub fn umt_trim_start_characters(s: &str, chars: &str) -> String {
    let char_set: Vec<char> = chars.chars().collect();
    s.chars().skip_while(|c| char_set.contains(c)).collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_trim_start_exclamation() {
        assert_eq!(umt_trim_start_characters("!!!hello", "!"), "hello");
    }

    #[test]
    fn test_trim_start_dashes() {
        assert_eq!(umt_trim_start_characters("---123", "-"), "123");
    }

    #[test]
    fn test_trim_start_no_match() {
        assert_eq!(umt_trim_start_characters("abc123", "xyz"), "abc123");
    }

    #[test]
    fn test_trim_start_empty() {
        assert_eq!(umt_trim_start_characters("", "!"), "");
    }

    #[test]
    fn test_trim_start_multiple_chars() {
        assert_eq!(umt_trim_start_characters("abcdef", "abc"), "def");
    }
}
