/// Escapes HTML special characters in a string
///
/// # Arguments
///
/// * `string_` - The string to escape
///
/// # Returns
///
/// The escaped string
///
/// # Examples
///
/// ```
/// use umt_rust::string::umt_escape_html;
///
/// assert_eq!(umt_escape_html("Tom & Jerry"), "Tom &amp; Jerry");
/// assert_eq!(umt_escape_html("<script>"), "&lt;script&gt;");
/// ```
#[inline]
pub fn umt_escape_html(string_: &str) -> String {
    let mut result = String::with_capacity(string_.len());
    for c in string_.chars() {
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
    fn test_escape_ampersand() {
        assert_eq!(umt_escape_html("Tom & Jerry"), "Tom &amp; Jerry");
    }

    #[test]
    fn test_escape_less_than() {
        assert_eq!(umt_escape_html("5 < 10"), "5 &lt; 10");
    }

    #[test]
    fn test_escape_greater_than() {
        assert_eq!(umt_escape_html("10 > 5"), "10 &gt; 5");
    }

    #[test]
    fn test_escape_double_quotes() {
        assert_eq!(umt_escape_html("Say \"Hello\""), "Say &quot;Hello&quot;");
    }

    #[test]
    fn test_escape_single_quotes() {
        assert_eq!(umt_escape_html("It's working"), "It&#39;s working");
    }

    #[test]
    fn test_escape_all_special_characters() {
        let input = "<script>alert(\"XSS & 'injection'\");</script>";
        let expected = "&lt;script&gt;alert(&quot;XSS &amp; &#39;injection&#39;&quot;);&lt;/script&gt;";
        assert_eq!(umt_escape_html(input), expected);
    }

    #[test]
    fn test_empty_string() {
        assert_eq!(umt_escape_html(""), "");
    }

    #[test]
    fn test_no_special_characters() {
        assert_eq!(umt_escape_html("Hello World"), "Hello World");
    }

    #[test]
    fn test_only_special_characters() {
        assert_eq!(umt_escape_html("&<>\"'"), "&amp;&lt;&gt;&quot;&#39;");
    }

    #[test]
    fn test_repeated_special_characters() {
        assert_eq!(umt_escape_html("&&&"), "&amp;&amp;&amp;");
        assert_eq!(umt_escape_html("<<<"), "&lt;&lt;&lt;");
    }
}
