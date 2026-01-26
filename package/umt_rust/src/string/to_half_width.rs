/// Convert full-width characters to half-width characters
///
/// # Arguments
/// * `s` - String to convert
///
/// # Returns
/// Converted string with full-width alphanumeric characters converted to half-width
///
/// # Example
/// ```
/// use umt_rust::string::umt_to_half_width;
/// assert_eq!(umt_to_half_width("ABC"), "ABC");
/// assert_eq!(umt_to_half_width("123"), "123");
/// ```
#[inline]
pub fn umt_to_half_width(s: &str) -> String {
    s.chars()
        .map(|c| {
            let code = c as u32;
            // Full-width digits: U+FF10 to U+FF19 (0-9)
            // Full-width uppercase: U+FF21 to U+FF3A (A-Z)
            // Full-width lowercase: U+FF41 to U+FF5A (a-z)
            if (0xFF10..=0xFF19).contains(&code)
                || (0xFF21..=0xFF3A).contains(&code)
                || (0xFF41..=0xFF5A).contains(&code)
            {
                // Convert by subtracting 0xFEE0
                char::from_u32(code - 0xFEE0).unwrap_or(c)
            } else {
                c
            }
        })
        .collect()
}
