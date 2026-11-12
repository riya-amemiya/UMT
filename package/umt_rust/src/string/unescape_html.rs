use regex::Regex;
use std::sync::LazyLock;

static ENTITY_REGEX: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(
        r"&(?:amp|lt|gt|quot|#39|#x27|#x2F|#x60|#x3D);|&#([0-9]{1,7});|&#x([0-9A-Fa-f]{1,6});",
    )
    .unwrap()
});

fn decode_code_point(code_point: u32) -> Option<char> {
    let is_unsafe = code_point == 0
        || code_point == 0x7F
        || (code_point != 0x09 && code_point != 0x0A && code_point != 0x0D && code_point <= 0x1F)
        || (0x80..=0x9F).contains(&code_point);
    if is_unsafe {
        None
    } else {
        char::from_u32(code_point)
    }
}

/// Unescapes HTML entities in a string
///
/// Numeric references that resolve to NULL, C0 controls other than TAB / LF /
/// CR, DEL, C1 controls, surrogates, or values above U+10FFFF are left
/// unchanged.
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

            let numeric = match (caps.get(1), caps.get(2)) {
                (Some(dec), _) => Some(dec.as_str().parse::<u32>()),
                (None, Some(hex)) => Some(u32::from_str_radix(hex.as_str(), 16)),
                (None, None) => None,
            };

            if let Some(parsed) = numeric {
                return parsed
                    .ok()
                    .and_then(decode_code_point)
                    .map_or_else(|| full_match.to_string(), |c| c.to_string());
            }

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
