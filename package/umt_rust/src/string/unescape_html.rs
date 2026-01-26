use regex::Regex;
use std::sync::LazyLock;

static ENTITY_REGEX: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(r"&(?:amp|lt|gt|quot|#39|#x27|#x2F|#x60|#x3D);|&#(\d+);|&#x([0-9a-fA-F]+);").unwrap()
});

/// Unescapes HTML entities in a string
///
/// # Arguments
/// * `s` - The string to unescape
///
/// # Returns
/// The unescaped string with HTML entities converted back to their original characters
///
/// # Example
/// ```
/// use umt_rust::string::umt_unescape_html;
/// assert_eq!(umt_unescape_html("&lt;script&gt;"), "<script>");
/// assert_eq!(umt_unescape_html("Tom &amp; Jerry"), "Tom & Jerry");
/// ```
#[inline]
pub fn umt_unescape_html(s: &str) -> String {
    ENTITY_REGEX
        .replace_all(s, |caps: &regex::Captures| {
            let full_match = caps.get(0).unwrap().as_str();

            // Check for decimal numeric entity &#123;
            if let Some(dec) = caps.get(1) {
                if let Ok(code_point) = dec.as_str().parse::<u32>()
                    && let Some(c) = char::from_u32(code_point)
                {
                    return c.to_string();
                }
                return full_match.to_string();
            }

            // Check for hex numeric entity &#x1F;
            if let Some(hex) = caps.get(2) {
                if let Ok(code_point) = u32::from_str_radix(hex.as_str(), 16)
                    && let Some(c) = char::from_u32(code_point)
                {
                    return c.to_string();
                }
                return full_match.to_string();
            }

            // Named entities
            match full_match {
                "&amp;" => "&".to_string(),
                "&lt;" => "<".to_string(),
                "&gt;" => ">".to_string(),
                "&quot;" => "\"".to_string(),
                "&#39;" | "&#x27;" => "'".to_string(),
                "&#x2F;" => "/".to_string(),
                "&#x60;" => "`".to_string(),
                "&#x3D;" => "=".to_string(),
                _ => full_match.to_string(),
            }
        })
        .to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_unescape_html_basic() {
        assert_eq!(umt_unescape_html("&lt;script&gt;"), "<script>");
    }

    #[test]
    fn test_unescape_html_ampersand() {
        assert_eq!(umt_unescape_html("Tom &amp; Jerry"), "Tom & Jerry");
    }

    #[test]
    fn test_unescape_html_quotes() {
        assert_eq!(umt_unescape_html("&quot;hello&quot;"), "\"hello\"");
        assert_eq!(umt_unescape_html("&#39;world&#39;"), "'world'");
    }

    #[test]
    fn test_unescape_html_numeric() {
        assert_eq!(umt_unescape_html("&#65;"), "A");
        assert_eq!(umt_unescape_html("&#x41;"), "A");
    }

    #[test]
    fn test_unescape_html_empty() {
        assert_eq!(umt_unescape_html(""), "");
    }
}
