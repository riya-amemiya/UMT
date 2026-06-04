use super::umt_words;

/// Converts a string to CONSTANT_CASE.
///
/// # Arguments
/// * `s` - The string to convert
///
/// # Returns
/// The CONSTANT_CASE string
///
/// # Examples
/// ```
/// use umt_rust::string::umt_constant_case;
/// assert_eq!(umt_constant_case("helloWorld"), "HELLO_WORLD");
/// assert_eq!(umt_constant_case("Hello World"), "HELLO_WORLD");
/// ```
#[inline]
pub fn umt_constant_case(s: &str) -> String {
    umt_words(s, None)
        .iter()
        .map(|word| word.to_uppercase())
        .collect::<Vec<_>>()
        .join("_")
}
