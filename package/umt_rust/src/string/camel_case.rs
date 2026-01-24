use regex::Regex;

/// Converts a string to camelCase
///
/// # Arguments
///
/// * `string_` - The string to convert
///
/// # Returns
///
/// The camelCase string
///
/// # Examples
///
/// ```
/// use umt_rust::string::umt_camel_case;
///
/// assert_eq!(umt_camel_case("hello-world"), "helloWorld");
/// assert_eq!(umt_camel_case("foo_bar_baz"), "fooBarBaz");
/// ```
#[inline]
pub fn umt_camel_case(string_: &str) -> String {
    // Replace non-alphanumeric sequences followed by a character with uppercase version
    let re1 = Regex::new(r"[^a-zA-Z0-9]+(.?)").unwrap();
    let result = re1.replace_all(string_, |caps: &regex::Captures| {
        caps.get(1)
            .map(|m| m.as_str().to_uppercase())
            .unwrap_or_default()
    });

    // Remove trailing non-alphanumeric characters
    let re2 = Regex::new(r"[^a-zA-Z0-9]+$").unwrap();
    let result = re2.replace_all(&result, "");

    // Lowercase the first character if it's uppercase
    let mut chars: Vec<char> = result.chars().collect();
    if let Some(first) = chars.first_mut() {
        if first.is_ascii_uppercase() {
            *first = first.to_ascii_lowercase();
        }
    }

    chars.into_iter().collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_kebab_case_to_camel_case() {
        assert_eq!(umt_camel_case("hello-world"), "helloWorld");
        assert_eq!(umt_camel_case("foo-bar-baz"), "fooBarBaz");
    }

    #[test]
    fn test_snake_case_to_camel_case() {
        assert_eq!(umt_camel_case("hello_world"), "helloWorld");
        assert_eq!(umt_camel_case("foo_bar_baz"), "fooBarBaz");
    }

    #[test]
    fn test_space_separated_to_camel_case() {
        assert_eq!(umt_camel_case("hello world"), "helloWorld");
        assert_eq!(umt_camel_case("foo bar baz"), "fooBarBaz");
    }

    #[test]
    fn test_pascal_case_input() {
        assert_eq!(umt_camel_case("HelloWorld"), "helloWorld");
        assert_eq!(umt_camel_case("FooBarBaz"), "fooBarBaz");
    }

    #[test]
    fn test_already_camel_case() {
        assert_eq!(umt_camel_case("helloWorld"), "helloWorld");
        assert_eq!(umt_camel_case("fooBarBaz"), "fooBarBaz");
    }

    #[test]
    fn test_mixed_separators() {
        assert_eq!(umt_camel_case("hello-world_test case"), "helloWorldTestCase");
        assert_eq!(umt_camel_case("foo_bar-baz qux"), "fooBarBazQux");
    }

    #[test]
    fn test_with_numbers() {
        assert_eq!(umt_camel_case("hello-world-2"), "helloWorld2");
        assert_eq!(umt_camel_case("test_case_123"), "testCase123");
    }

    #[test]
    fn test_empty_string() {
        assert_eq!(umt_camel_case(""), "");
    }

    #[test]
    fn test_single_word() {
        assert_eq!(umt_camel_case("hello"), "hello");
        assert_eq!(umt_camel_case("HELLO"), "hELLO");
    }

    #[test]
    fn test_special_characters() {
        assert_eq!(umt_camel_case("hello@world"), "helloWorld");
        assert_eq!(umt_camel_case("foo#bar$baz"), "fooBarBaz");
    }

    #[test]
    fn test_leading_trailing_separators() {
        assert_eq!(umt_camel_case("-hello-world-"), "helloWorld");
        assert_eq!(umt_camel_case("_foo_bar_"), "fooBar");
    }

    #[test]
    fn test_multiple_consecutive_separators() {
        assert_eq!(umt_camel_case("hello---world"), "helloWorld");
        assert_eq!(umt_camel_case("foo___bar"), "fooBar");
    }
}
