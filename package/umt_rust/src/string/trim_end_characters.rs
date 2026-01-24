/// Removes specified characters from the end of a string
///
/// # Arguments
///
/// * `string_` - The input string to trim
/// * `chars` - Characters to remove from the end
///
/// # Returns
///
/// A new string with specified characters removed from the end
///
/// # Examples
///
/// ```
/// use umt_rust::string::umt_trim_end_characters;
///
/// assert_eq!(umt_trim_end_characters("hello!!!", "!"), "hello");
/// assert_eq!(umt_trim_end_characters("123---", "-"), "123");
/// ```
#[inline]
pub fn umt_trim_end_characters(string_: &str, chars: &str) -> String {
    let chars_set: Vec<char> = chars.chars().collect();
    let string_chars: Vec<char> = string_.chars().collect();

    let mut end_index = string_chars.len();
    while end_index > 0 && chars_set.contains(&string_chars[end_index - 1]) {
        end_index -= 1;
    }

    string_chars[..end_index].iter().collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_remove_from_end() {
        assert_eq!(umt_trim_end_characters("hellooo", "o"), "hell");
        assert_eq!(umt_trim_end_characters("banana!!!", "!"), "banana");
    }

    #[test]
    fn test_multiple_different_characters() {
        assert_eq!(umt_trim_end_characters("hello123...", "123."), "hello");
        assert_eq!(umt_trim_end_characters("test---...", ".-"), "test");
    }

    #[test]
    fn test_no_characters_match() {
        assert_eq!(umt_trim_end_characters("apple", "x"), "apple");
        assert_eq!(umt_trim_end_characters("test", "xyz"), "test");
    }

    #[test]
    fn test_empty_input() {
        assert_eq!(umt_trim_end_characters("", "x"), "");
        assert_eq!(umt_trim_end_characters("", ""), "");
    }

    #[test]
    fn test_all_characters_trimmed() {
        assert_eq!(umt_trim_end_characters("xxxxx", "x"), "");
        assert_eq!(umt_trim_end_characters("....", "."), "");
    }

    #[test]
    fn test_empty_trim_characters() {
        assert_eq!(umt_trim_end_characters("hello", ""), "hello");
        assert_eq!(umt_trim_end_characters("test123", ""), "test123");
    }

    #[test]
    fn test_non_ascii_characters() {
        assert_eq!(
            umt_trim_end_characters("\u{3053}\u{3093}\u{306B}\u{3061}\u{306F}\u{3002}\u{3002}\u{3002}", "\u{3002}"),
            "\u{3053}\u{3093}\u{306B}\u{3061}\u{306F}"
        );
        assert_eq!(umt_trim_end_characters("Hello\u{FF01}\u{FF01}", "\u{FF01}"), "Hello");
    }
}
