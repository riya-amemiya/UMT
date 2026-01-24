/// Removes all whitespace characters from a string
///
/// # Arguments
///
/// * `string_` - Input string
///
/// # Returns
///
/// String with all whitespace characters removed
///
/// # Examples
///
/// ```
/// use umt_rust::string::umt_delete_spaces;
///
/// assert_eq!(umt_delete_spaces("Hello World"), "HelloWorld");
/// assert_eq!(umt_delete_spaces("  tab\t space "), "tabspace");
/// ```
#[inline]
pub fn umt_delete_spaces(string_: &str) -> String {
    string_.chars().filter(|c| !c.is_whitespace()).collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_remove_spaces() {
        assert_eq!(umt_delete_spaces("Hello World"), "HelloWorld");
        assert_eq!(umt_delete_spaces("   Hello   World   "), "HelloWorld");
    }

    #[test]
    fn test_remove_tabs_and_whitespace() {
        assert_eq!(umt_delete_spaces("Hello\tWorld"), "HelloWorld");
        assert_eq!(umt_delete_spaces("Hello\nWorld\r\n"), "HelloWorld");
        // EM space (U+2003)
        assert_eq!(umt_delete_spaces("Hello\u{2003}World"), "HelloWorld");
    }

    #[test]
    fn test_empty_string() {
        assert_eq!(umt_delete_spaces(""), "");
        assert_eq!(umt_delete_spaces("   "), "");
    }

    #[test]
    fn test_multibyte_characters() {
        assert_eq!(umt_delete_spaces("\u{3053}\u{3093}\u{306B}\u{3061}\u{306F} \u{4E16}\u{754C}"), "\u{3053}\u{3093}\u{306B}\u{3061}\u{306F}\u{4E16}\u{754C}");
        // Full-width space (U+3000)
        assert_eq!(umt_delete_spaces("Hello\u{3000}World"), "HelloWorld");
    }
}
