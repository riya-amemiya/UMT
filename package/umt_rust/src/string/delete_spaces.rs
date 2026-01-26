/// Removes all whitespace characters from a string
///
/// # Arguments
/// * `s` - Input string
///
/// # Returns
/// String with all whitespace characters removed
///
/// # Example
/// ```
/// use umt_rust::string::umt_delete_spaces;
/// assert_eq!(umt_delete_spaces("Hello World"), "HelloWorld");
/// assert_eq!(umt_delete_spaces("  tab\t space "), "tabspace");
/// ```
#[inline]
pub fn umt_delete_spaces(s: &str) -> String {
    s.chars().filter(|c| !c.is_whitespace()).collect()
}
