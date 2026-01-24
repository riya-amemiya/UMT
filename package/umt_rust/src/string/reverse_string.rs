/// Reverses a string
///
/// # Arguments
///
/// * `char_` - String to reverse
///
/// # Returns
///
/// Reversed string
///
/// # Examples
///
/// ```
/// use umt_rust::string::umt_reverse_string;
///
/// assert_eq!(umt_reverse_string("Hello"), "olleH");
/// assert_eq!(umt_reverse_string("abc"), "cba");
/// ```
#[inline]
pub fn umt_reverse_string(char_: &str) -> String {
    char_.chars().rev().collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_three_characters() {
        assert_eq!(umt_reverse_string("abc"), "cba");
    }

    #[test]
    fn test_empty_string() {
        assert_eq!(umt_reverse_string(""), "");
    }

    #[test]
    fn test_single_character() {
        assert_eq!(umt_reverse_string("a"), "a");
    }

    #[test]
    fn test_two_characters() {
        assert_eq!(umt_reverse_string("ab"), "ba");
    }

    #[test]
    fn test_four_characters() {
        assert_eq!(umt_reverse_string("abcd"), "dcba");
    }

    #[test]
    fn test_five_characters() {
        assert_eq!(umt_reverse_string("abcde"), "edcba");
    }

    #[test]
    fn test_six_characters() {
        assert_eq!(umt_reverse_string("abcdef"), "fedcba");
    }

    #[test]
    fn test_seven_characters() {
        assert_eq!(umt_reverse_string("abcdefg"), "gfedcba");
    }

    #[test]
    fn test_eight_characters() {
        assert_eq!(umt_reverse_string("abcdefgh"), "hgfedcba");
    }

    #[test]
    fn test_whitespace() {
        assert_eq!(umt_reverse_string("hello world"), "dlrow olleh");
        assert_eq!(umt_reverse_string("  "), "  ");
    }

    #[test]
    fn test_special_characters() {
        assert_eq!(umt_reverse_string("!@#$%"), "%$#@!");
        assert_eq!(umt_reverse_string("a-b-c"), "c-b-a");
    }

    #[test]
    fn test_numbers_in_string() {
        assert_eq!(umt_reverse_string("12345"), "54321");
        assert_eq!(umt_reverse_string("a1b2c3"), "3c2b1a");
    }

    #[test]
    fn test_unicode_characters() {
        assert_eq!(umt_reverse_string("\u{3042}\u{3044}\u{3046}"), "\u{3046}\u{3044}\u{3042}");
        assert_eq!(umt_reverse_string("\u{65E5}\u{672C}\u{8A9E}"), "\u{8A9E}\u{672C}\u{65E5}");
    }
}
