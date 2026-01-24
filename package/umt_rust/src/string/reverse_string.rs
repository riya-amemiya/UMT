/// Reverses a string
///
/// # Arguments
/// * `s` - String to reverse
///
/// # Returns
/// Reversed string
///
/// # Example
/// ```
/// use umt_rust::string::umt_reverse_string;
/// assert_eq!(umt_reverse_string("Hello"), "olleH");
/// ```
#[inline]
pub fn umt_reverse_string(s: &str) -> String {
    s.chars().rev().collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_reverse_string_basic() {
        assert_eq!(umt_reverse_string("Hello"), "olleH");
    }

    #[test]
    fn test_reverse_string_empty() {
        assert_eq!(umt_reverse_string(""), "");
    }

    #[test]
    fn test_reverse_string_single() {
        assert_eq!(umt_reverse_string("a"), "a");
    }

    #[test]
    fn test_reverse_string_palindrome() {
        assert_eq!(umt_reverse_string("racecar"), "racecar");
    }
}
