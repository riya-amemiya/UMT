/// Adds the specified string to the end of the string until it reaches the specified length
///
/// Lengths are counted in characters.
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
    let length = s.chars().count();
    if pad_string.is_empty() || length >= target_length {
        return s.to_string();
    }

    let padding_length = target_length - length;
    let mut result = String::with_capacity(s.len() + padding_length);
    result.push_str(s);
    result.extend(pad_string.chars().cycle().take(padding_length));
    result
}
