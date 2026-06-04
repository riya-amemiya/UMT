use super::umt_words;

/// Converts a string to snake_case.
///
/// # Arguments
/// * `s` - The string to convert
///
/// # Returns
/// The snake_case string
///
/// # Examples
/// ```
/// use umt_rust::string::umt_snake_case;
/// assert_eq!(umt_snake_case("helloWorld"), "hello_world");
/// assert_eq!(umt_snake_case("XMLHttpRequest"), "xml_http_request");
/// ```
#[inline]
pub fn umt_snake_case(s: &str) -> String {
    umt_words(s, None)
        .iter()
        .map(|word| word.to_lowercase())
        .collect::<Vec<_>>()
        .join("_")
}
