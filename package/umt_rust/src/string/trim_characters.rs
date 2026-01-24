use crate::string::{umt_trim_end_characters, umt_trim_start_characters};

/// Removes specified characters from both ends of a string.
///
/// # Arguments
///
/// * `string_` - The string to trim.
/// * `chars` - The set of characters to remove.
///
/// # Returns
///
/// A new string with specified characters removed from both ends.
///
/// # Examples
///
/// ```
/// use umt_rust::string::umt_trim_characters;
///
/// assert_eq!(umt_trim_characters("---Hello World---", "-"), "Hello World");
/// ```
#[inline]
pub fn umt_trim_characters(string_: &str, chars: &str) -> String {
    let trimmed_start = umt_trim_start_characters(string_, chars);
    umt_trim_end_characters(&trimmed_start, chars)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_remove_from_both_ends() {
        assert_eq!(umt_trim_characters("---Hello World---", "-"), "Hello World");
    }

    #[test]
    fn test_not_remove_from_middle() {
        assert_eq!(umt_trim_characters("---Hello-World---", "-"), "Hello-World");
    }

    #[test]
    fn test_empty_input() {
        assert_eq!(umt_trim_characters("", "-"), "");
    }

    #[test]
    fn test_empty_chars() {
        assert_eq!(umt_trim_characters("Hello World", ""), "Hello World");
    }
}
