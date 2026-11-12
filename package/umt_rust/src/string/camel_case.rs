use regex::{Captures, Regex};
use std::sync::LazyLock;

static NON_ALNUM_FOLLOWED_RE: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(r"[^0-9A-Za-z]+([^\n\r\u{2028}\u{2029}])").unwrap());
static TRAILING_NON_ALNUM_RE: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(r"[^0-9A-Za-z]+$").unwrap());

/// Converts a string to camelCase
///
/// Every run of characters other than ASCII letters and digits is removed and
/// the character that follows it is uppercased. Only an ASCII uppercase first
/// character is lowercased.
///
/// # Arguments
/// * `s` - The string to convert
///
/// # Returns
/// The camelCase string
///
/// # Example
/// ```
/// use umt_rust::string::umt_camel_case;
/// assert_eq!(umt_camel_case("hello-world"), "helloWorld");
/// assert_eq!(umt_camel_case("foo_bar_baz"), "fooBarBaz");
/// ```
#[inline]
pub fn umt_camel_case(s: &str) -> String {
    let replaced = NON_ALNUM_FOLLOWED_RE.replace_all(s, |caps: &Captures| caps[1].to_uppercase());
    let trimmed = TRAILING_NON_ALNUM_RE.replace(&replaced, "");

    let mut chars = trimmed.chars();
    match chars.next() {
        Some(first) if first.is_ascii_uppercase() => {
            let mut result = String::with_capacity(trimmed.len());
            result.push(first.to_ascii_lowercase());
            result.push_str(chars.as_str());
            result
        }
        _ => trimmed.into_owned(),
    }
}
