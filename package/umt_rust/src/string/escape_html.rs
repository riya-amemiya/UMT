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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_escape_html_basic() {
        assert_eq!(umt_escape_html("<script>"), "&lt;script&gt;");
    }

    #[test]
    fn test_escape_html_ampersand() {
        assert_eq!(umt_escape_html("Tom & Jerry"), "Tom &amp; Jerry");
    }

    #[test]
    fn test_escape_html_quotes() {
        assert_eq!(umt_escape_html("\"hello\""), "&quot;hello&quot;");
        assert_eq!(umt_escape_html("'world'"), "&#39;world&#39;");
    }

    #[test]
    fn test_escape_html_empty() {
        assert_eq!(umt_escape_html(""), "");
    }

    #[test]
    fn test_escape_html_no_special() {
        assert_eq!(umt_escape_html("hello"), "hello");
    }
}
