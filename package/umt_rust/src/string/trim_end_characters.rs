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
    let char_set: Vec<char> = chars.chars().collect();
    let s_chars: Vec<char> = s.chars().collect();

    let mut end_index = s_chars.len();
    while end_index > 0 && char_set.contains(&s_chars[end_index - 1]) {
        end_index -= 1;
    }

    s_chars[..end_index].iter().collect()
}
