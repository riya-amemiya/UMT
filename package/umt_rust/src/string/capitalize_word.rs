/// Capitalizes the first character of a word and lowercases the rest.
///
/// # Arguments
/// * `word` - The word to transform
///
/// # Returns
/// The word with its first character uppercased and the remainder lowercased
///
/// # Examples
/// ```
/// use umt_rust::string::umt_capitalize_word;
/// assert_eq!(umt_capitalize_word("hello"), "Hello");
/// assert_eq!(umt_capitalize_word("hELLO"), "Hello");
/// assert_eq!(umt_capitalize_word(""), "");
/// ```
#[inline]
pub fn umt_capitalize_word(word: &str) -> String {
    let mut chars = word.chars();
    match chars.next() {
        Some(first) => first.to_uppercase().collect::<String>() + &chars.as_str().to_lowercase(),
        None => String::new(),
    }
}
