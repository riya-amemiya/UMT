use regex::Regex;

/// Checks whether a URL string is absolute (RFC 3986).
///
/// An absolute URL starts with a scheme followed by a colon,
/// where the scheme begins with a letter and may contain
/// letters, digits, plus, hyphen, or period.
///
/// # Arguments
///
/// * `url` - The URL string to check
///
/// # Returns
///
/// `true` if the URL is absolute, `false` otherwise.
///
/// # Example
///
/// ```
/// use umt_rust::url::umt_is_absolute_url;
///
/// assert!(umt_is_absolute_url("https://example.com"));
/// assert!(umt_is_absolute_url("ftp://files.example"));
/// assert!(!umt_is_absolute_url("/path/to/page"));
/// assert!(!umt_is_absolute_url("relative/path"));
/// assert!(umt_is_absolute_url("mailto:user@host"));
/// ```
pub fn umt_is_absolute_url(url: &str) -> bool {
    let re = Regex::new(r"(?i)^[a-z][\da-z+.\-]*:").unwrap();
    re.is_match(url)
}
