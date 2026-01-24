use regex::Regex;
use std::collections::HashMap;

/// Unescapes HTML entities in a string
///
/// # Arguments
///
/// * `string_` - The string to unescape
///
/// # Returns
///
/// The unescaped string with HTML entities converted back to their original characters
///
/// # Examples
///
/// ```
/// use umt_rust::string::umt_unescape_html;
///
/// assert_eq!(umt_unescape_html("Tom &amp; Jerry"), "Tom & Jerry");
/// assert_eq!(umt_unescape_html("5 &lt; 10"), "5 < 10");
/// ```
#[inline]
pub fn umt_unescape_html(string_: &str) -> String {
    let html_unescape_map: HashMap<&str, &str> = [
        ("&amp;", "&"),
        ("&lt;", "<"),
        ("&gt;", ">"),
        ("&quot;", "\""),
        ("&#39;", "'"),
        ("&#x27;", "'"),
        ("&#x2F;", "/"),
        ("&#x60;", "`"),
        ("&#x3D;", "="),
    ]
    .into_iter()
    .collect();

    let entity_regex =
        Regex::new(r"&(?:amp|lt|gt|quot|#39|#x27|#x2F|#x60|#x3D);|&#(\d+);|&#x([0-9a-fA-F]+);")
            .unwrap();

    entity_regex
        .replace_all(string_, |caps: &regex::Captures| {
            // Check for decimal numeric character reference
            if let Some(dec_match) = caps.get(1) {
                let dec_str = dec_match.as_str();
                if let Ok(code_point) = dec_str.parse::<u32>() {
                    if let Some(c) = char::from_u32(code_point) {
                        return c.to_string();
                    }
                }
                return caps[0].to_string();
            }

            // Check for hexadecimal numeric character reference
            if let Some(hex_match) = caps.get(2) {
                let hex_str = hex_match.as_str();
                if let Ok(code_point) = u32::from_str_radix(hex_str, 16) {
                    if let Some(c) = char::from_u32(code_point) {
                        return c.to_string();
                    }
                }
                return caps[0].to_string();
            }

            // Check for named entity
            let matched = &caps[0];
            html_unescape_map
                .get(matched)
                .map(|&s| s.to_string())
                .unwrap_or_else(|| matched.to_string())
        })
        .to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_unescape_ampersand() {
        assert_eq!(umt_unescape_html("Tom &amp; Jerry"), "Tom & Jerry");
    }

    #[test]
    fn test_unescape_less_than() {
        assert_eq!(umt_unescape_html("5 &lt; 10"), "5 < 10");
    }

    #[test]
    fn test_unescape_greater_than() {
        assert_eq!(umt_unescape_html("10 &gt; 5"), "10 > 5");
    }

    #[test]
    fn test_unescape_double_quotes() {
        assert_eq!(umt_unescape_html("Say &quot;Hello&quot;"), "Say \"Hello\"");
    }

    #[test]
    fn test_unescape_single_quotes() {
        assert_eq!(umt_unescape_html("It&#39;s working"), "It's working");
    }

    #[test]
    fn test_unescape_hex_single_quotes() {
        assert_eq!(umt_unescape_html("It&#x27;s working"), "It's working");
    }

    #[test]
    fn test_unescape_all_basic_entities() {
        let input =
            "&lt;script&gt;alert(&quot;XSS &amp; &#39;injection&#39;&quot;);&lt;/script&gt;";
        let expected = "<script>alert(\"XSS & 'injection'\");</script>";
        assert_eq!(umt_unescape_html(input), expected);
    }

    #[test]
    fn test_empty_string() {
        assert_eq!(umt_unescape_html(""), "");
    }

    #[test]
    fn test_no_entities() {
        assert_eq!(umt_unescape_html("Hello World"), "Hello World");
    }

    #[test]
    fn test_only_entities() {
        assert_eq!(umt_unescape_html("&amp;&lt;&gt;&quot;&#39;"), "&<>\"'");
    }

    #[test]
    fn test_repeated_entities() {
        assert_eq!(umt_unescape_html("&amp;&amp;&amp;"), "&&&");
        assert_eq!(umt_unescape_html("&lt;&lt;&lt;"), "<<<");
    }

    #[test]
    fn test_decimal_numeric_references() {
        assert_eq!(umt_unescape_html("&#65;"), "A");
        assert_eq!(umt_unescape_html("&#8364;"), "\u{20AC}");
        assert_eq!(umt_unescape_html("&#128512;"), "\u{1F600}");
    }

    #[test]
    fn test_hexadecimal_numeric_references() {
        assert_eq!(umt_unescape_html("&#x41;"), "A");
        assert_eq!(umt_unescape_html("&#x20AC;"), "\u{20AC}");
        assert_eq!(umt_unescape_html("&#x1F600;"), "\u{1F600}");
    }

    #[test]
    fn test_mixed_case_hex_references() {
        assert_eq!(umt_unescape_html("&#x41;"), "A");
        // Uppercase X should not match
        assert_eq!(umt_unescape_html("&#X41;"), "&#X41;");
        assert_eq!(umt_unescape_html("&#x20ac;"), "\u{20AC}");
        assert_eq!(umt_unescape_html("&#x20AC;"), "\u{20AC}");
    }

    #[test]
    fn test_extended_entities() {
        assert_eq!(umt_unescape_html("&#x2F;"), "/");
        assert_eq!(umt_unescape_html("&#x60;"), "`");
        assert_eq!(umt_unescape_html("&#x3D;"), "=");
    }

    #[test]
    fn test_invalid_numeric_references() {
        assert_eq!(umt_unescape_html("&#;"), "&#;");
        assert_eq!(umt_unescape_html("&#x;"), "&#x;");
    }

    #[test]
    fn test_malformed_entities() {
        assert_eq!(umt_unescape_html("&lt"), "&lt");
        assert_eq!(umt_unescape_html("&unknown;"), "&unknown;");
        assert_eq!(umt_unescape_html("&#"), "&#");
    }

    #[test]
    fn test_unicode_numeric_references() {
        assert_eq!(umt_unescape_html("&#12354;"), "\u{3042}");
        assert_eq!(umt_unescape_html("&#x3042;"), "\u{3042}");
        assert_eq!(umt_unescape_html("&#8226;"), "\u{2022}");
        assert_eq!(umt_unescape_html("&#x2022;"), "\u{2022}");
    }

    #[test]
    fn test_preserve_unescaped_content() {
        let input = "Already < unescaped & content with 'quotes'";
        assert_eq!(umt_unescape_html(input), input);
    }
}
