/// Lowercases the first character of a string. Does not change the rest.
///
/// Iterates by Unicode scalar value, so characters outside the Basic
/// Multilingual Plane (e.g. emoji) are treated as a single unit.
///
/// # Arguments
/// * `s` - The string to uncapitalize
///
/// # Returns
/// The string with its first character lowercased
///
/// # Examples
/// ```
/// use umt_rust::string::umt_uncapitalize;
/// assert_eq!(umt_uncapitalize("Hello"), "hello");
/// assert_eq!(umt_uncapitalize("ÉCLAIR"), "éCLAIR");
/// assert_eq!(umt_uncapitalize(""), "");
/// ```
#[inline]
pub fn umt_uncapitalize(s: &str) -> String {
    let mut chars = s.chars();
    match chars.next() {
        Some(first) => first.to_lowercase().collect::<String>() + chars.as_str(),
        None => String::new(),
    }
}
