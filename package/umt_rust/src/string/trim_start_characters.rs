/// Removes specified characters from the start of a string
///
/// # Arguments
/// * `s` - The input string to trim
/// * `chars` - Characters to remove from the start
///
/// # Returns
/// A new string with specified characters removed from the start
///
/// # Example
/// ```
/// use umt_rust::string::umt_trim_start_characters;
/// assert_eq!(umt_trim_start_characters("!!!hello", "!"), "hello");
/// assert_eq!(umt_trim_start_characters("---123", "-"), "123");
/// assert_eq!(umt_trim_start_characters("abc123", "xyz"), "abc123");
/// ```
#[inline]
pub fn umt_trim_start_characters(s: &str, chars: &str) -> String {
    s.trim_start_matches(|c: char| chars.contains(c))
        .to_string()
}
