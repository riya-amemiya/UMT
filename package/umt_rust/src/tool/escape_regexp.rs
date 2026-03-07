/// Escapes characters in a string for use in a regular expression.
///
/// # Arguments
///
/// * `string` - The string to escape.
///
/// # Returns
///
/// The escaped string.
///
/// # Example
///
/// ```
/// use umt_rust::tool::umt_escape_regexp;
///
/// let escaped = umt_escape_regexp("a.b");
/// assert_eq!(escaped, "a\\.b");
/// ```
pub fn umt_escape_regexp(string: &str) -> String {
    let mut result = String::with_capacity(string.len() + 10);
    for c in string.chars() {
        match c {
            '$' | '(' | ')' | '*' | '+' | '.' | '?' | '[' | '\\' | ']' | '^' | '{' | '|' | '}' => {
                result.push('\\');
                result.push(c);
            }
            _ => {
                result.push(c);
            }
        }
    }
    result
}
