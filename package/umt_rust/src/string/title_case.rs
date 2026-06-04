use super::{umt_capitalize_word, umt_words};

/// Converts a string to Title Case (each word capitalized, separated by spaces).
///
/// # Arguments
/// * `s` - The string to convert
///
/// # Returns
/// The Title Case string
///
/// # Examples
/// ```
/// use umt_rust::string::umt_title_case;
/// assert_eq!(umt_title_case("hello world"), "Hello World");
/// assert_eq!(umt_title_case("a-quick brown_fox"), "A Quick Brown Fox");
/// ```
#[inline]
pub fn umt_title_case(s: &str) -> String {
    umt_words(s, None)
        .iter()
        .map(|word| umt_capitalize_word(word))
        .collect::<Vec<_>>()
        .join(" ")
}
