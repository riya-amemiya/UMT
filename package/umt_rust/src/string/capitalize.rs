/// Capitalizes the first character of a string. Does not lowercase the rest.
///
/// Iterates by Unicode scalar value, so characters outside the Basic
/// Multilingual Plane (e.g. emoji) are treated as a single unit.
///
/// # Arguments
/// * `s` - The string to capitalize
///
/// # Returns
/// The string with its first character uppercased
///
/// # Examples
/// ```
/// use umt_rust::string::umt_capitalize;
/// assert_eq!(umt_capitalize("hello"), "Hello");
/// assert_eq!(umt_capitalize("éclair"), "Éclair");
/// assert_eq!(umt_capitalize(""), "");
/// ```
#[inline]
pub fn umt_capitalize(s: &str) -> String {
    let mut chars = s.chars();
    match chars.next() {
        Some(first) => first.to_uppercase().collect::<String>() + chars.as_str(),
        None => String::new(),
    }
}
