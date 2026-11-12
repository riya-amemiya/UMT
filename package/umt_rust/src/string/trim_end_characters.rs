/// Removes specified characters from the end of a string
///
/// # Arguments
/// * `s` - The input string to trim
/// * `chars` - Characters to remove from the end
///
/// # Returns
/// A new string with specified characters removed from the end
///
/// # Example
/// ```
/// use umt_rust::string::umt_trim_end_characters;
/// assert_eq!(umt_trim_end_characters("hello!!!", "!"), "hello");
/// assert_eq!(umt_trim_end_characters("123---", "-"), "123");
/// assert_eq!(umt_trim_end_characters("abc123", "xyz"), "abc123");
/// ```
#[inline]
pub fn umt_trim_end_characters(s: &str, chars: &str) -> String {
    s.trim_end_matches(|c: char| chars.contains(c)).to_string()
}
