<<<<<<< HEAD
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
||||||| 55b8153
=======
/// Error type for unwrap failures.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct UnwrapError {
    /// The error message.
    pub message: String,
}

impl std::fmt::Display for UnwrapError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.message)
    }
}

impl std::error::Error for UnwrapError {}

/// Unwraps a value that may be None, returning an error if the value is absent.
///
/// # Arguments
///
/// * `value` - The value to unwrap (may be None)
/// * `message` - The error message to return if the value is absent
///
/// # Returns
///
/// Returns `Ok(T)` if the value is Some, or `Err(UnwrapError)` if the value is None.
///
/// # Examples
///
/// ```
/// use umt_rust::tool::umt_unwrap;
///
/// let value: Option<i32> = Some(42);
/// let result = umt_unwrap(value, "Value was None");
/// assert_eq!(result.unwrap(), 42);
///
/// let none_value: Option<i32> = None;
/// let result = umt_unwrap(none_value, "Value was None");
/// assert!(result.is_err());
/// assert_eq!(result.unwrap_err().message, "Value was None");
/// ```
pub fn umt_unwrap<T>(value: Option<T>, message: &str) -> Result<T, UnwrapError> {
    match value {
        Some(v) => Ok(v),
        None => Err(UnwrapError {
            message: message.to_string(),
        }),
    }
}

/// Unwraps a value that may be None, panicking if the value is absent.
///
/// This is similar to `Option::expect()` but with a custom error type.
///
/// # Arguments
///
/// * `value` - The value to unwrap (may be None)
/// * `message` - The error message to include in the panic if the value is absent
///
/// # Returns
///
/// Returns the unwrapped value of type T.
///
/// # Panics
///
/// Panics if the value is None.
///
/// # Examples
///
/// ```
/// use umt_rust::tool::umt_unwrap_or_panic;
///
/// let value: Option<i32> = Some(42);
/// let result = umt_unwrap_or_panic(value, "Value was None");
/// assert_eq!(result, 42);
/// ```
///
/// ```should_panic
/// use umt_rust::tool::umt_unwrap_or_panic;
///
/// let none_value: Option<i32> = None;
/// umt_unwrap_or_panic(none_value, "Value was None"); // This will panic
/// ```
pub fn umt_unwrap_or_panic<T>(value: Option<T>, message: &str) -> T {
    match value {
        Some(v) => v,
        None => panic!("{}", message),
    }
}

/// Unwraps a value or returns a default if the value is absent.
///
/// # Arguments
///
/// * `value` - The value to unwrap (may be None)
/// * `default` - The default value to return if the value is absent
///
/// # Returns
///
/// Returns the unwrapped value or the default.
///
/// # Examples
///
/// ```
/// use umt_rust::tool::umt_unwrap_or;
///
/// let value: Option<i32> = Some(42);
/// assert_eq!(umt_unwrap_or(value, 0), 42);
///
/// let none_value: Option<i32> = None;
/// assert_eq!(umt_unwrap_or(none_value, 0), 0);
/// ```
pub fn umt_unwrap_or<T>(value: Option<T>, default: T) -> T {
    value.unwrap_or(default)
}

/// Unwraps a value or computes a default using the provided closure.
///
/// # Arguments
///
/// * `value` - The value to unwrap (may be None)
/// * `f` - A closure that computes the default value
///
/// # Returns
///
/// Returns the unwrapped value or the computed default.
///
/// # Examples
///
/// ```
/// use umt_rust::tool::umt_unwrap_or_else;
///
/// let value: Option<i32> = Some(42);
/// assert_eq!(umt_unwrap_or_else(value, || 0), 42);
///
/// let none_value: Option<i32> = None;
/// assert_eq!(umt_unwrap_or_else(none_value, || 100 + 1), 101);
/// ```
pub fn umt_unwrap_or_else<T, F>(value: Option<T>, f: F) -> T
where
    F: FnOnce() -> T,
{
    value.unwrap_or_else(f)
}
>>>>>>> d916e13f83cf5d012a5cda2956ae6890fbe99788
