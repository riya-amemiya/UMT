/// Escapes HTML special characters in a string
///
/// # Arguments
/// * `s` - The string to escape
///
/// # Returns
/// The escaped string
///
/// # Example
/// ```
/// use umt_rust::string::umt_escape_html;
/// assert_eq!(umt_escape_html("<script>"), "&lt;script&gt;");
/// assert_eq!(umt_escape_html("Tom & Jerry"), "Tom &amp; Jerry");
/// ```
#[inline]
pub fn umt_escape_html(s: &str) -> String {
    let mut result = String::with_capacity(s.len());
    for c in s.chars() {
        match c {
            '&' => result.push_str("&amp;"),
            '<' => result.push_str("&lt;"),
            '>' => result.push_str("&gt;"),
            '"' => result.push_str("&quot;"),
            '\'' => result.push_str("&#39;"),
            _ => result.push(c),
        }
    }
    result
}
