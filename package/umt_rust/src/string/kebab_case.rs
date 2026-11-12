/// Converts a string to kebab-case
///
/// A dash is inserted between an ASCII lowercase letter and an uppercase
/// letter, and before the last uppercase letter of an acronym that is followed
/// by a lowercase letter. Every run of characters other than ASCII letters and
/// digits becomes a single dash, and leading and trailing dashes are removed.
///
/// # Arguments
/// * `s` - The string to convert
///
/// # Returns
/// The kebab-case string
///
/// # Example
/// ```
/// use umt_rust::string::umt_kebab_case;
/// assert_eq!(umt_kebab_case("helloWorld"), "hello-world");
/// assert_eq!(umt_kebab_case("foo_bar_baz"), "foo-bar-baz");
/// ```
#[inline]
pub fn umt_kebab_case(s: &str) -> String {
    let bytes = s.as_bytes();
    let mut result = String::with_capacity(s.len() + 8);
    let mut pending_dash = false;

    for (index, &byte) in bytes.iter().enumerate() {
        if !byte.is_ascii_alphanumeric() {
            pending_dash = true;
            continue;
        }

        if byte.is_ascii_uppercase() && index > 0 {
            let previous = bytes[index - 1];
            if previous.is_ascii_lowercase()
                || (previous.is_ascii_uppercase()
                    && bytes.get(index + 1).is_some_and(u8::is_ascii_lowercase))
            {
                pending_dash = true;
            }
        }

        if pending_dash && !result.is_empty() {
            result.push('-');
        }
        pending_dash = false;
        result.push(byte.to_ascii_lowercase() as char);
    }

    result
}
