/// Adds the specified string to the end of the string until it reaches the specified length
///
/// # Arguments
/// * `s` - The original string to apply padding
/// * `target_length` - The target length after padding
/// * `pad_string` - The string to use for padding
///
/// # Returns
/// The string after padding has been applied
///
/// # Example
/// ```
/// use umt_rust::string::umt_pad_end;
/// assert_eq!(umt_pad_end("123", 5, "0"), "12300");
/// assert_eq!(umt_pad_end("abc", 8, "def"), "abcdefde");
/// ```
#[inline]
pub fn umt_pad_end(s: &str, target_length: usize, pad_string: &str) -> String {
    if pad_string.is_empty() {
        return s.to_string();
    }

    let mut result = s.to_string();
    while result.len() < target_length {
        let remaining = target_length - result.len();
        let chars_to_add: String = pad_string.chars().take(remaining).collect();
        result.push_str(&chars_to_add);
    }
    result
}
