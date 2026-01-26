use crate::internal::unicode_nfd;

/// Convert a string to a URL-friendly slug
///
/// # Arguments
/// * `s` - The string to convert
///
/// # Returns
/// The slugified string
///
/// # Example
/// ```
/// use umt_rust::string::umt_slugify;
/// assert_eq!(umt_slugify("Hello World!"), "hello-world");
/// assert_eq!(umt_slugify("This is a Test"), "this-is-a-test");
/// ```
#[inline]
pub fn umt_slugify(s: &str) -> String {
    // NFD normalization and remove combining marks
    let normalized = unicode_nfd::strip_diacritics(s);

    let mut result = String::with_capacity(normalized.len());
    let mut prev_was_dash = true; // Start as true to avoid leading dash

    for c in normalized.chars() {
        if c.is_alphanumeric() {
            result.push(c.to_ascii_lowercase());
            prev_was_dash = false;
        } else {
            if !prev_was_dash {
                result.push('-');
                prev_was_dash = true;
            }
        }
    }

    // Remove trailing dash
    if result.ends_with('-') {
        result.pop();
    }

    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_slugify_basic() {
        assert_eq!(umt_slugify("Hello World!"), "hello-world");
    }

    #[test]
    fn test_slugify_sentence() {
        assert_eq!(umt_slugify("This is a Test"), "this-is-a-test");
    }

    #[test]
    fn test_slugify_special_chars() {
        assert_eq!(umt_slugify("Hello, World!"), "hello-world");
    }

    #[test]
    fn test_slugify_empty() {
        assert_eq!(umt_slugify(""), "");
    }

    #[test]
    fn test_slugify_accented() {
        assert_eq!(umt_slugify("cafe"), "cafe");
    }
}
