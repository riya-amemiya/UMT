/// Swaps the case of each letter in a string.
///
/// Uppercase letters become lowercase and vice versa. Characters without case
/// (digits, punctuation, CJK ideographs, etc.) are left unchanged.
///
/// # Arguments
/// * `s` - The string to transform
///
/// # Returns
/// The string with swapped case
///
/// # Examples
/// ```
/// use umt_rust::string::umt_swap_case;
/// assert_eq!(umt_swap_case("Hello World"), "hELLO wORLD");
/// assert_eq!(umt_swap_case("abc123-日本"), "ABC123-日本");
/// ```
pub fn umt_swap_case(s: &str) -> String {
    let mut result = String::with_capacity(s.len());
    for c in s.chars() {
        if c.is_alphabetic() {
            let lower: String = c.to_lowercase().collect();
            if lower == c.to_string() {
                result.extend(c.to_uppercase());
            } else {
                result.extend(c.to_lowercase());
            }
        } else {
            result.push(c);
        }
    }
    result
}
