use regex::Regex;
use unicode_normalization::UnicodeNormalization;

/// Convert a string to a URL-friendly slug
///
/// # Arguments
///
/// * `string_` - The string to convert
///
/// # Returns
///
/// The slugified string
///
/// # Examples
///
/// ```
/// use umt_rust::string::umt_slugify;
///
/// assert_eq!(umt_slugify("Hello World!"), "hello-world");
/// assert_eq!(umt_slugify("This is a Test"), "this-is-a-test");
/// ```
#[inline]
pub fn umt_slugify(string_: &str) -> String {
    // Normalize to NFD and remove combining diacritical marks
    let normalized: String = string_
        .nfd()
        .filter(|c| !('\u{0300}'..='\u{036F}').contains(c))
        .collect();

    let lower = normalized.to_lowercase();

    // Replace non-word characters (except whitespace and dash) with dash
    let re1 = Regex::new(r"[^\w\s-]").unwrap();
    let result = re1.replace_all(&lower, "-");

    // Replace whitespace with dash
    let re2 = Regex::new(r"\s+").unwrap();
    let result = re2.replace_all(&result, "-");

    // Replace underscores with dash
    let re3 = Regex::new(r"_+").unwrap();
    let result = re3.replace_all(&result, "-");

    // Remove multiple consecutive dashes
    let re4 = Regex::new(r"-+").unwrap();
    let result = re4.replace_all(&result, "-");

    // Remove leading and trailing dashes
    let re5 = Regex::new(r"^-+|-+$").unwrap();
    let result = re5.replace_all(&result, "");

    result.to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_jsdoc_examples() {
        assert_eq!(umt_slugify("Hello World!"), "hello-world");
        assert_eq!(umt_slugify("This is a Test"), "this-is-a-test");
        assert_eq!(umt_slugify("Japanese: \u{3053}\u{3093}\u{306B}\u{3061}\u{306F}"), "japanese");
    }

    #[test]
    fn test_multiple_spaces() {
        assert_eq!(umt_slugify("Hello    World"), "hello-world");
    }

    #[test]
    fn test_leading_trailing_spaces() {
        assert_eq!(umt_slugify("  Hello World  "), "hello-world");
    }

    #[test]
    fn test_special_characters() {
        assert_eq!(umt_slugify("Special!@#$%Characters"), "special-characters");
    }

    #[test]
    fn test_underscores() {
        assert_eq!(umt_slugify("snake_case_text"), "snake-case-text");
    }

    #[test]
    fn test_unicode_characters() {
        assert_eq!(umt_slugify("caf\u{E9}"), "cafe");
        assert_eq!(umt_slugify("na\u{EF}ve"), "naive");
    }

    #[test]
    fn test_numbers() {
        assert_eq!(umt_slugify("Test 123"), "test-123");
        assert_eq!(umt_slugify("Version 2.5"), "version-2-5");
    }

    #[test]
    fn test_empty_string() {
        assert_eq!(umt_slugify(""), "");
    }

    #[test]
    fn test_consecutive_hyphens() {
        assert_eq!(umt_slugify("Hello---World"), "hello-world");
    }

    #[test]
    fn test_mixed_case() {
        assert_eq!(umt_slugify("CamelCase"), "camelcase");
    }
}
