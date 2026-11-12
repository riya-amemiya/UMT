/// Pads the start of a string with another string until the target length is reached
///
/// Lengths are counted in characters.
///
/// # Arguments
/// * `s` - The original string to pad
/// * `target_length` - The target length after padding
/// * `pad_string` - The string to use for padding
///
/// # Returns
/// The padded string
///
/// # Example
/// ```
/// use umt_rust::string::umt_pad_start;
/// assert_eq!(umt_pad_start("123", 5, "0"), "00123");
/// assert_eq!(umt_pad_start("abc", 8, "def"), "defdeabc");
/// ```
#[inline]
pub fn umt_pad_start(s: &str, target_length: usize, pad_string: &str) -> String {
    let length = s.chars().count();
    if pad_string.is_empty() || length >= target_length {
        return s.to_string();
    }

    let padding_length = target_length - length;
    let mut result = String::with_capacity(s.len() + padding_length);
    result.extend(pad_string.chars().cycle().take(padding_length));
    result.push_str(s);
    result
}
