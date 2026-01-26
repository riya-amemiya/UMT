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
    let mut result = String::with_capacity(s.len());
    let mut chars = s.chars().peekable();

    while let Some(c) = chars.next() {
        if c != '&' {
            result.push(c);
            continue;
        }

        // Collect entity
        let mut entity = String::from("&");
        let mut found_semicolon = false;
        for _ in 0..10 {
            match chars.next() {
                Some(';') => {
                    entity.push(';');
                    found_semicolon = true;
                    break;
                }
                Some(ch) => entity.push(ch),
                None => break,
            }
        }

        if !found_semicolon {
            result.push_str(&entity);
            continue;
        }

        match entity.as_str() {
            "&amp;" => result.push('&'),
            "&lt;" => result.push('<'),
            "&gt;" => result.push('>'),
            "&quot;" => result.push('"'),
            "&#39;" | "&#x27;" => result.push('\''),
            "&#x2F;" => result.push('/'),
            "&#x60;" => result.push('`'),
            "&#x3D;" => result.push('='),
            _ => {
                // Try decimal numeric entity &#123;
                if entity.starts_with("&#x") {
                    let hex_str = &entity[3..entity.len() - 1];
                    if let Ok(code_point) = u32::from_str_radix(hex_str, 16) {
                        if let Some(c) = char::from_u32(code_point) {
                            result.push(c);
                            continue;
                        }
                    }
                } else if entity.starts_with("&#") {
                    let dec_str = &entity[2..entity.len() - 1];
                    if let Ok(code_point) = dec_str.parse::<u32>() {
                        if let Some(c) = char::from_u32(code_point) {
                            result.push(c);
                            continue;
                        }
                    }
                }
                result.push_str(&entity);
            }
        }
    }

    result
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
