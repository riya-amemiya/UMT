use super::{umt_capitalize, umt_words};

/// Converts a string to Sentence case
///
/// The first word is capitalized, every other word is lower case, and all words
/// are separated by single spaces. Words are split with `umt_words`.
///
/// # Arguments
/// * `s` - The string to convert
///
/// # Returns
/// The Sentence case string
///
/// # Example
/// ```
/// use umt_rust::string::umt_sentence_case;
/// assert_eq!(umt_sentence_case("helloWorld foo-bar"), "Hello world foo bar");
/// assert_eq!(umt_sentence_case("XMLHttpRequest"), "Xml http request");
/// assert_eq!(umt_sentence_case(""), "");
/// ```
pub fn umt_sentence_case(s: &str) -> String {
    umt_words(s, None)
        .iter()
        .enumerate()
        .map(|(index, word)| {
            let lower = word.to_lowercase();
            if index == 0 {
                umt_capitalize(&lower)
            } else {
                lower
            }
        })
        .collect::<Vec<_>>()
        .join(" ")
}
