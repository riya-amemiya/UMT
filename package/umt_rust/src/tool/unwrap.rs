/// Unwraps a value that may be absent, panicking with a message if it is.
///
/// # Arguments
///
/// * `value` - The option to unwrap
/// * `message` - The error message to panic with if the value is None
///
/// # Returns
///
/// Returns the unwrapped value.
///
/// # Panics
///
/// Panics if the value is None.
pub fn umt_unwrap<T>(value: Option<T>, message: &str) -> T {
    value.expect(message)
}
