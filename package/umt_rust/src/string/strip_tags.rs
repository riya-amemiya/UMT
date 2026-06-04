use regex::Regex;

/// Removes HTML/XML tags from a string, keeping the text content.
///
/// Unlike escaping, this deletes the tags entirely. The removal repeats until
/// the string is stable so that nested constructs (e.g. `"<sc<script>ript>"`)
/// cannot survive a single pass.
///
/// # Arguments
/// * `s` - The string to strip
///
/// # Returns
/// The string with tags removed
///
/// # Examples
/// ```
/// use umt_rust::string::umt_strip_tags;
/// assert_eq!(umt_strip_tags("<p>Hello <b>World</b></p>"), "Hello World");
/// assert_eq!(umt_strip_tags("<sc<script>ript>"), "");
/// ```
pub fn umt_strip_tags(s: &str) -> String {
    let tag = Regex::new(r"<[^<>]*>").unwrap();
    let mut result = s.to_string();
    loop {
        let next = tag.replace_all(&result, "").to_string();
        if next == result {
            break;
        }
        result = next;
    }
    result
}
