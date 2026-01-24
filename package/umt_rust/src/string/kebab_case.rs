use regex::Regex;

/// Converts a string to kebab-case
///
/// # Arguments
///
/// * `string_` - The string to convert
///
/// # Returns
///
/// The kebab-case string
///
/// # Examples
///
/// ```
/// use umt_rust::string::umt_kebab_case;
///
/// assert_eq!(umt_kebab_case("helloWorld"), "hello-world");
/// assert_eq!(umt_kebab_case("FooBarBaz"), "foo-bar-baz");
/// ```
#[inline]
pub fn umt_kebab_case(string_: &str) -> String {
    // Insert dash between lowercase and uppercase
    let re1 = Regex::new(r"([a-z])([A-Z])").unwrap();
    let result = re1.replace_all(string_, "$1-$2");

    // Insert dash between sequences of uppercase letters and following lowercase
    let re2 = Regex::new(r"([A-Z])([A-Z][a-z])").unwrap();
    let result = re2.replace_all(&result, "$1-$2");

    // Replace spaces and underscores with dashes
    let re3 = Regex::new(r"[\s_]+").unwrap();
    let result = re3.replace_all(&result, "-");

    // Remove special characters except alphanumeric and dashes
    let re4 = Regex::new(r"[^a-zA-Z0-9-]").unwrap();
    let result = re4.replace_all(&result, "-");

    // Remove multiple consecutive dashes
    let re5 = Regex::new(r"-+").unwrap();
    let result = re5.replace_all(&result, "-");

    // Remove leading and trailing dashes
    let re6 = Regex::new(r"^-|-$").unwrap();
    let result = re6.replace_all(&result, "");

    result.to_lowercase()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_camel_case_to_kebab() {
        assert_eq!(umt_kebab_case("helloWorld"), "hello-world");
        assert_eq!(umt_kebab_case("fooBarBaz"), "foo-bar-baz");
    }

    #[test]
    fn test_pascal_case_to_kebab() {
        assert_eq!(umt_kebab_case("HelloWorld"), "hello-world");
        assert_eq!(umt_kebab_case("FooBarBaz"), "foo-bar-baz");
    }

    #[test]
    fn test_snake_case_to_kebab() {
        assert_eq!(umt_kebab_case("hello_world"), "hello-world");
        assert_eq!(umt_kebab_case("foo_bar_baz"), "foo-bar-baz");
    }

    #[test]
    fn test_space_separated_to_kebab() {
        assert_eq!(umt_kebab_case("hello world"), "hello-world");
        assert_eq!(umt_kebab_case("foo bar baz"), "foo-bar-baz");
    }

    #[test]
    fn test_already_kebab_case() {
        assert_eq!(umt_kebab_case("hello-world"), "hello-world");
        assert_eq!(umt_kebab_case("foo-bar-baz"), "foo-bar-baz");
    }

    #[test]
    fn test_mixed_separators() {
        assert_eq!(umt_kebab_case("helloWorld_test case"), "hello-world-test-case");
        assert_eq!(umt_kebab_case("fooBar-baz qux"), "foo-bar-baz-qux");
    }

    #[test]
    fn test_with_numbers() {
        assert_eq!(umt_kebab_case("helloWorld2"), "hello-world2");
        assert_eq!(umt_kebab_case("testCase123"), "test-case123");
    }

    #[test]
    fn test_empty_string() {
        assert_eq!(umt_kebab_case(""), "");
    }

    #[test]
    fn test_single_word() {
        assert_eq!(umt_kebab_case("hello"), "hello");
        assert_eq!(umt_kebab_case("HELLO"), "hello");
    }

    #[test]
    fn test_remove_special_characters() {
        assert_eq!(umt_kebab_case("hello@world"), "hello-world");
        assert_eq!(umt_kebab_case("foo#bar$baz"), "foo-bar-baz");
    }

    #[test]
    fn test_leading_trailing_separators() {
        assert_eq!(umt_kebab_case("-hello-world-"), "hello-world");
        assert_eq!(umt_kebab_case("_foo_bar_"), "foo-bar");
    }

    #[test]
    fn test_multiple_consecutive_separators() {
        assert_eq!(umt_kebab_case("hello---world"), "hello-world");
        assert_eq!(umt_kebab_case("foo___bar"), "foo-bar");
    }

    #[test]
    fn test_complex_mixed_case() {
        assert_eq!(umt_kebab_case("XMLHttpRequest"), "xml-http-request");
        assert_eq!(umt_kebab_case("getElementById"), "get-element-by-id");
    }

    #[test]
    fn test_acronyms() {
        assert_eq!(umt_kebab_case("HTML"), "html");
        assert_eq!(umt_kebab_case("XMLParser"), "xml-parser");
    }
}
