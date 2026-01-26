/// Converts a string to kebab-case
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
    let mut result = String::with_capacity(s.len() + 10);
    let mut prev_was_lower = false;
    let mut prev_was_upper = false;
    let mut prev_was_dash = true; // Start as true to avoid leading dash

    for c in s.chars() {
        if c.is_alphanumeric() {
            if c.is_uppercase() {
                if prev_was_lower {
                    result.push('-');
                }
                result.push(c.to_ascii_lowercase());
                prev_was_upper = true;
                prev_was_lower = false;
            } else {
                if prev_was_upper && result.len() > 1 {
                    // Handle cases like "HTMLParser" -> "html-parser"
                    let last_char = result.pop().unwrap();
                    if !result.ends_with('-') {
                        result.push('-');
                    }
                    result.push(last_char);
                }
                result.push(c.to_ascii_lowercase());
                prev_was_lower = c.is_lowercase();
                prev_was_upper = false;
            }
            prev_was_dash = false;
        } else {
            if !prev_was_dash && !result.is_empty() {
                result.push('-');
                prev_was_dash = true;
            }
            prev_was_lower = false;
            prev_was_upper = false;
        }
    }

    // Remove trailing dash
    if result.ends_with('-') {
        result.pop();
    }

    // Remove multiple consecutive dashes
    let mut final_result = String::with_capacity(result.len());
    let mut prev_was_dash = false;
    for c in result.chars() {
        if c == '-' {
            if !prev_was_dash {
                final_result.push(c);
                prev_was_dash = true;
            }
        } else {
            final_result.push(c);
            prev_was_dash = false;
        }
    }

    final_result
}
