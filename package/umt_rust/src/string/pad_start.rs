/// Pads the start of a string with another string until the target length is reached
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
    if pad_string.is_empty() || s.len() >= target_length {
        return s.to_string();
    }

    let padding_length = target_length - s.len();
    let mut padding = String::with_capacity(padding_length);

    while padding.len() < padding_length {
        padding.push_str(pad_string);
    }

    // Trim padding to exact length needed
    let trimmed_padding: String = padding.chars().take(padding_length).collect();
    format!("{}{}", trimmed_padding, s)
}
