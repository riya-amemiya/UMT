/// Pads the start of a string with another string until the target length is reached
///
/// # Arguments
///
/// * `string_` - The original string to pad
/// * `target_length` - The target length after padding
/// * `pad_string` - The string to use for padding
///
/// # Returns
///
/// The padded string
///
/// # Examples
///
/// ```
/// use umt_rust::string::umt_pad_start;
///
/// assert_eq!(umt_pad_start("123", 5, "0"), "00123");
/// assert_eq!(umt_pad_start("abc", 8, "def"), "defdeabc");
/// ```
#[inline]
pub fn umt_pad_start(string_: &str, target_length: usize, pad_string: &str) -> String {
    let current_len = string_.chars().count();

    // Return original string if padString is empty or string is already long enough
    if pad_string.is_empty() || current_len >= target_length {
        return string_.to_string();
    }

    let padding_length = target_length - current_len;
    let pad_chars: Vec<char> = pad_string.chars().collect();
    let mut padding = String::new();

    // Build padding by repeating padString
    while padding.chars().count() < padding_length {
        padding.push_str(pad_string);
    }

    // Trim padding to exact length needed and concatenate with original string
    let trimmed_padding: String = padding.chars().take(padding_length).collect();
    format!("{}{}", trimmed_padding, string_)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_basic_padding() {
        assert_eq!(umt_pad_start("abc", 5, " "), "  abc");
        assert_eq!(umt_pad_start("abc", 5, "0"), "00abc");
    }

    #[test]
    fn test_already_at_or_exceeding_target() {
        assert_eq!(umt_pad_start("abc", 2, " "), "abc"); // Shorter target
        assert_eq!(umt_pad_start("abc", 3, " "), "abc"); // Equal length
        assert_eq!(umt_pad_start("abc", 0, " "), "abc"); // Zero target
    }

    #[test]
    fn test_empty_string_input() {
        assert_eq!(umt_pad_start("", 3, "0"), "000");
        assert_eq!(umt_pad_start("", 4, "x"), "xxxx");
    }

    #[test]
    fn test_multi_character_padding() {
        assert_eq!(umt_pad_start("abc", 7, "xy"), "xyxyabc");
        assert_eq!(umt_pad_start("abc", 8, "123"), "12312abc");
    }

    #[test]
    fn test_truncate_padding() {
        assert_eq!(umt_pad_start("abc", 6, "12345"), "123abc");
        assert_eq!(umt_pad_start("abc", 5, "0000"), "00abc");
    }

    #[test]
    fn test_single_character_padding() {
        assert_eq!(umt_pad_start("abc", 5, "x"), "xxabc");
    }

    #[test]
    fn test_empty_padding_string() {
        assert_eq!(umt_pad_start("abc", 5, ""), "abc");
        assert_eq!(umt_pad_start("test", 10, ""), "test");
    }
}
