use unicode_normalization::UnicodeNormalization;

/// Convert a string to a URL-friendly slug
///
/// The string is NFD-normalized, combining diacritical marks are removed, and
/// every run of characters other than ASCII letters and digits becomes a
/// single dash. Leading and trailing dashes are removed.
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
    let mut result = String::with_capacity(s.len());
    let mut pending_dash = false;

    for c in s.nfd() {
        if ('\u{0300}'..='\u{036F}').contains(&c) {
            continue;
        }
        if c.is_ascii_alphanumeric() {
            if pending_dash && !result.is_empty() {
                result.push('-');
            }
            pending_dash = false;
            result.push(c.to_ascii_lowercase());
        } else {
            pending_dash = true;
        }
    }

    result
}
