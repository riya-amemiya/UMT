/// Converts half-width ASCII alphanumeric characters to full-width.
///
/// Inverse of half-width conversion; only ASCII digits, uppercase, and lowercase
/// letters are converted by adding `0xFEE0` to their code point. Other
/// characters are left unchanged.
///
/// # Arguments
/// * `s` - The string to convert
///
/// # Returns
/// The converted string
///
/// # Examples
/// ```
/// use umt_rust::string::umt_to_full_width;
/// assert_eq!(umt_to_full_width("Abc123"), "Ａｂｃ１２３");
/// assert_eq!(umt_to_full_width("a-b!"), "ａ-ｂ!");
/// ```
#[inline]
pub fn umt_to_full_width(s: &str) -> String {
    s.chars()
        .map(|c| {
            if c.is_ascii_digit() || c.is_ascii_alphabetic() {
                char::from_u32(c as u32 + 0xFEE0).unwrap_or(c)
            } else {
                c
            }
        })
        .collect()
}
