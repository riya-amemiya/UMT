use super::{umt_capitalize_word, umt_words};

/// Converts a string to PascalCase.
///
/// # Arguments
/// * `s` - The string to convert
///
/// # Returns
/// The PascalCase string
///
/// # Examples
/// ```
/// use umt_rust::string::umt_pascal_case;
/// assert_eq!(umt_pascal_case("hello-world"), "HelloWorld");
/// assert_eq!(umt_pascal_case("hello_world"), "HelloWorld");
/// ```
#[inline]
pub fn umt_pascal_case(s: &str) -> String {
    umt_words(s, None)
        .iter()
        .map(|word| umt_capitalize_word(word))
        .collect()
}
