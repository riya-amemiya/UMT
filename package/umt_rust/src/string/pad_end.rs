/// Adds the specified string to the end of the string until it reaches the specified length.
///
/// # Arguments
///
/// * `string_` - The original string to apply padding
/// * `target_length` - The target length after padding
/// * `pad_string` - The string to use for padding
///
/// # Returns
///
/// The string after padding has been applied
///
/// # Examples
///
/// ```
/// use umt_rust::string::umt_pad_end;
///
/// assert_eq!(umt_pad_end("abc", 5, " "), "abc  ");
/// assert_eq!(umt_pad_end("hello", 10, "!"), "hello!!!!!");
/// ```
#[inline]
pub fn umt_pad_end(string_: &str, target_length: usize, pad_string: &str) -> String {
    if pad_string.is_empty() {
        return string_.to_string();
    }

    let current_len = string_.chars().count();
    if current_len >= target_length {
        return string_.to_string();
    }

    let mut result = string_.to_string();
    let pad_chars: Vec<char> = pad_string.chars().collect();

    while result.chars().count() < target_length {
        let remaining = target_length - result.chars().count();
        let take_count = remaining.min(pad_chars.len());
        result.extend(pad_chars.iter().take(take_count));
    }

    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_add_padding_to_end() {
        assert_eq!(umt_pad_end("abc", 5, " "), "abc  ");
        assert_eq!(umt_pad_end("hello", 10, "!"), "hello!!!!!");
    }

    #[test]
    fn test_no_modify_if_already_long_enough() {
        assert_eq!(umt_pad_end("abc", 3, " "), "abc");
        assert_eq!(umt_pad_end("longstring", 5, "!"), "longstring");
    }

    #[test]
    fn test_multi_character_padding() {
        assert_eq!(umt_pad_end("abc", 10, "de"), "abcdededed");
    }

    #[test]
    fn test_shorter_target_length() {
        assert_eq!(umt_pad_end("abc", 2, " "), "abc");
    }

    #[test]
    fn test_empty_padding_string() {
        assert_eq!(umt_pad_end("abc", 5, ""), "abc");
    }
}
