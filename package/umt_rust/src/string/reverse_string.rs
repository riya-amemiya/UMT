/// Reverses a string
///
/// # Arguments
/// * `s` - String to reverse
///
/// # Returns
/// Reversed string
///
/// # Example
/// ```
/// use umt_rust::string::umt_reverse_string;
/// assert_eq!(umt_reverse_string("Hello"), "olleH");
/// ```
#[inline]
pub fn umt_reverse_string(s: &str) -> String {
    s.chars().rev().collect()
}
