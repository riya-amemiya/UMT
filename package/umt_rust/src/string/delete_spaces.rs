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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_delete_spaces_basic() {
        assert_eq!(umt_delete_spaces("Hello World"), "HelloWorld");
    }

    #[test]
    fn test_delete_spaces_tabs() {
        assert_eq!(umt_delete_spaces("  tab\t space "), "tabspace");
    }

    #[test]
    fn test_delete_spaces_empty() {
        assert_eq!(umt_delete_spaces(""), "");
    }

    #[test]
    fn test_delete_spaces_no_spaces() {
        assert_eq!(umt_delete_spaces("HelloWorld"), "HelloWorld");
    }
}
