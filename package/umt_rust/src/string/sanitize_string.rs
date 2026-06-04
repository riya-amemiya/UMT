/// Sanitizes a string by removing non-printable characters.
///
/// Keeps the printable ASCII range (space `U+0020` through tilde `U+007E`) plus
/// tab, line feed, and carriage return. Everything else, including ANSI escape
/// bytes, NULL bytes, other control characters, and non-ASCII characters, is
/// removed.
///
/// # Arguments
/// * `s` - The string to sanitize
///
/// # Returns
/// The sanitized string
///
/// # Examples
/// ```
/// use umt_rust::string::umt_sanitize_string;
/// assert_eq!(umt_sanitize_string("Hello, World!"), "Hello, World!");
/// assert_eq!(umt_sanitize_string("a\u{0000}b"), "ab");
/// ```
#[inline]
pub fn umt_sanitize_string(s: &str) -> String {
    s.chars()
        .filter(|&c| c == '\t' || c == '\n' || c == '\r' || (' '..='~').contains(&c))
        .collect()
}
