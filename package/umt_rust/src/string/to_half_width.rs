/// Convert full-width characters to half-width characters
///
/// # Arguments
///
/// * `string_` - String to convert
///
/// # Returns
///
/// Converted string
///
/// # Examples
///
/// ```
/// use umt_rust::string::umt_to_half_width;
///
/// assert_eq!(umt_to_half_width("\u{FF10}\u{FF11}\u{FF12}\u{FF13}\u{FF14}"), "01234");
/// assert_eq!(umt_to_half_width("\u{FF21}\u{FF22}\u{FF23}"), "ABC");
/// ```
#[inline]
pub fn umt_to_half_width(string_: &str) -> String {
    string_
        .chars()
        .map(|c| {
            let code = c as u32;
            // Full-width digits: \uFF10 - \uFF19 (0-9)
            // Full-width uppercase: \uFF21 - \uFF3A (A-Z)
            // Full-width lowercase: \uFF41 - \uFF5A (a-z)
            if (0xFF10..=0xFF19).contains(&code)
                || (0xFF21..=0xFF3A).contains(&code)
                || (0xFF41..=0xFF5A).contains(&code)
            {
                // Convert to half-width by subtracting 0xFEE0
                char::from_u32(code - 0xFEE0).unwrap_or(c)
            } else {
                c
            }
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_full_width_alphanumeric_to_half_width() {
        assert_eq!(
            umt_to_half_width("\u{FF10}\u{FF11}\u{FF12}\u{FF13}\u{FF14}\u{FF15}\u{FF16}\u{FF17}\u{FF18}\u{FF19}"),
            "0123456789"
        );
        assert_eq!(
            umt_to_half_width("\u{FF21}\u{FF22}\u{FF23}\u{FF24}\u{FF25}\u{FF26}\u{FF27}\u{FF28}\u{FF29}\u{FF2A}"),
            "ABCDEFGHIJ"
        );
        assert_eq!(
            umt_to_half_width("\u{FF41}\u{FF42}\u{FF43}\u{FF44}\u{FF45}\u{FF46}\u{FF47}\u{FF48}\u{FF49}\u{FF4A}"),
            "abcdefghij"
        );
    }

    #[test]
    fn test_mixed_full_width_and_half_width() {
        assert_eq!(
            umt_to_half_width("\u{FF21}\u{FF22}\u{FF23}abc\u{FF11}\u{FF12}\u{FF13}123"),
            "ABCabc123123"
        );
    }

    #[test]
    fn test_keep_non_target_characters() {
        assert_eq!(
            umt_to_half_width("\u{6F22}\u{5B57}\u{30AB}\u{30BF}\u{30AB}\u{30CA}\u{3001}\u{3002}"),
            "\u{6F22}\u{5B57}\u{30AB}\u{30BF}\u{30AB}\u{30CA}\u{3001}\u{3002}"
        );
    }
}
