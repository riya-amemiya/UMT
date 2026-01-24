/// Removes specified characters from the start of a string
///
/// # Arguments
///
/// * `string_` - The input string to trim
/// * `chars` - Characters to remove from the start
///
/// # Returns
///
/// A new string with specified characters removed from the start
///
/// # Examples
///
/// ```
/// use umt_rust::string::umt_trim_start_characters;
///
/// assert_eq!(umt_trim_start_characters("!!!hello", "!"), "hello");
/// assert_eq!(umt_trim_start_characters("---123", "-"), "123");
/// ```
#[inline]
pub fn umt_trim_start_characters(string_: &str, chars: &str) -> String {
    let chars_set: Vec<char> = chars.chars().collect();
    let string_chars: Vec<char> = string_.chars().collect();

    let mut start_index = 0;
    while start_index < string_chars.len() && chars_set.contains(&string_chars[start_index]) {
        start_index += 1;
    }

    string_chars[start_index..].iter().collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_remove_from_start() {
        assert_eq!(umt_trim_start_characters("---Hello", "-"), "Hello");
        assert_eq!(umt_trim_start_characters("!!!world", "!"), "world");
    }

    #[test]
    fn test_multiple_different_characters() {
        assert_eq!(umt_trim_start_characters("...123test", ".123"), "test");
        assert_eq!(umt_trim_start_characters("---...text", ".-"), "text");
    }

    #[test]
    fn test_no_characters_match() {
        assert_eq!(umt_trim_start_characters("hello", "x"), "hello");
        assert_eq!(umt_trim_start_characters("test", "xyz"), "test");
    }

    #[test]
    fn test_empty_input() {
        assert_eq!(umt_trim_start_characters("", "x"), "");
        assert_eq!(umt_trim_start_characters("", ""), "");
    }

    #[test]
    fn test_all_characters_trimmed() {
        assert_eq!(umt_trim_start_characters("xxxxx", "x"), "");
        assert_eq!(umt_trim_start_characters(".....", "."), "");
    }

    #[test]
    fn test_empty_trim_characters() {
        assert_eq!(umt_trim_start_characters("hello", ""), "hello");
        assert_eq!(umt_trim_start_characters("123test", ""), "123test");
    }

    #[test]
    fn test_non_ascii_characters() {
        assert_eq!(
            umt_trim_start_characters("\u{3002}\u{3002}\u{3002}\u{3053}\u{3093}\u{306B}\u{3061}\u{306F}", "\u{3002}"),
            "\u{3053}\u{3093}\u{306B}\u{3061}\u{306F}"
        );
        assert_eq!(umt_trim_start_characters("\u{FF01}\u{FF01}Hello", "\u{FF01}"), "Hello");
    }
}
