/// Converts a string to camelCase
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
    let mut result = String::with_capacity(s.len());
    let mut capitalize_next = false;
    let mut first_letter_found = false;

    for c in s.chars() {
        if c.is_alphanumeric() {
            if capitalize_next {
                result.push(c.to_ascii_uppercase());
                capitalize_next = false;
            } else if !first_letter_found {
                result.push(c.to_ascii_lowercase());
                first_letter_found = true;
            } else {
                result.push(c);
            }
        } else {
            if first_letter_found {
                capitalize_next = true;
            }
        }
    }

    result
}
