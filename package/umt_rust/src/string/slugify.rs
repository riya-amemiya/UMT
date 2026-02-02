use unicode_normalization::UnicodeNormalization;

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
    let normalized: String = s
        .nfd()
        .filter(|c| !('\u{0300}'..='\u{036F}').contains(c))
        .collect();

    let mut result = String::with_capacity(normalized.len());
    let mut prev_was_dash = true; // Start as true to avoid leading dash

    for c in normalized.chars() {
        if c.is_alphanumeric() {
            result.push(c.to_ascii_lowercase());
            prev_was_dash = false;
        } else if !prev_was_dash {
            result.push('-');
            prev_was_dash = true;
        }
    }

    // Remove trailing dash
    if result.ends_with('-') {
        result.pop();
    }

    result
}
