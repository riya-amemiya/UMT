//! String type checking utility

/// Determines if the value is a string
///
/// In Rust, this function accepts a &str or String reference and always returns true
/// due to Rust's type system.
///
/// # Arguments
/// * `_value` - The string reference to check
///
/// # Returns
/// Always returns true since the input is typed as &str
///
/// # Examples
/// ```
/// use umt_rust::validate::umt_is_string;
///
/// assert!(umt_is_string("test"));
/// assert!(umt_is_string(&String::from("hello")));
/// ```
#[inline]
pub fn umt_is_string(_value: &str) -> bool {
    true
}
