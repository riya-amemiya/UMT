use super::umt_words;

/// Counts words in a string using the same boundaries as [`umt_words`].
///
/// # Arguments
/// * `s` - The string to inspect
///
/// # Returns
/// The number of words
///
/// # Examples
/// ```
/// use umt_rust::string::umt_word_count;
/// assert_eq!(umt_word_count("hello world"), 2);
/// assert_eq!(umt_word_count("camelCase split"), 3);
/// ```
#[inline]
pub fn umt_word_count(s: &str) -> usize {
    umt_words(s, None).len()
}
