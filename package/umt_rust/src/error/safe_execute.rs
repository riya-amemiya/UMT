use std::panic::{AssertUnwindSafe, catch_unwind};

/// Represents a successful execution result containing a value
#[derive(Debug, Clone, PartialEq)]
pub struct SuccessType<V> {
    pub value: V,
}

/// Represents an error result containing an error message
#[derive(Debug, Clone, PartialEq)]
pub struct ErrorType<E> {
    pub error: E,
}

/// Union type representing either a successful result or an error result
/// Similar to TypeScript's `{ type: "success", value } | { type: "error", error }`
#[derive(Debug, Clone, PartialEq)]
pub enum SafeResult<V, E> {
    Success(SuccessType<V>),
    Error(ErrorType<E>),
}

impl<V, E> SafeResult<V, E> {
    /// Returns true if the result is a success
    pub fn is_success(&self) -> bool {
        matches!(self, SafeResult::Success(_))
    }

    /// Returns true if the result is an error
    pub fn is_error(&self) -> bool {
        matches!(self, SafeResult::Error(_))
    }

    /// Returns the success value if present
    pub fn value(&self) -> Option<&V> {
        match self {
            SafeResult::Success(s) => Some(&s.value),
            SafeResult::Error(_) => None,
        }
    }

    /// Returns the error if present
    pub fn error(&self) -> Option<&E> {
        match self {
            SafeResult::Success(_) => None,
            SafeResult::Error(e) => Some(&e.error),
        }
    }

    /// Converts to a standard Result type
    pub fn to_result(self) -> Result<V, E> {
        match self {
            SafeResult::Success(s) => Ok(s.value),
            SafeResult::Error(e) => Err(e.error),
        }
    }

    /// Returns the type as a string ("success" or "error")
    pub fn result_type(&self) -> &'static str {
        match self {
            SafeResult::Success(_) => "success",
            SafeResult::Error(_) => "error",
        }
    }
}

/// Creates a success result
fn success<V>(value: V) -> SafeResult<V, String> {
    SafeResult::Success(SuccessType { value })
}

/// Creates an error result
fn error<V>(err: String) -> SafeResult<V, String> {
    SafeResult::Error(ErrorType { error: err })
}

/// Safely executes a callback function and returns a SafeResult type.
/// Catches any panics and wraps them in a SafeResult type.
///
/// # Arguments
///
/// * `callback` - The function to execute safely
///
/// # Returns
///
/// A SafeResult containing either the successful value or the caught error message
///
/// # Example
///
/// ```
/// use umt_rust::error::umt_safe_execute;
///
/// // Successful execution
/// let result = umt_safe_execute(|| 42);
/// assert!(result.is_success());
/// assert_eq!(result.value(), Some(&42));
///
/// // Panicking execution
/// let result = umt_safe_execute(|| -> i32 {
///     panic!("test error");
/// });
/// assert!(result.is_error());
/// ```
#[inline]
pub fn umt_safe_execute<V, F>(callback: F) -> SafeResult<V, String>
where
    F: FnOnce() -> V + std::panic::UnwindSafe,
{
    match catch_unwind(callback) {
        Ok(value) => success(value),
        Err(panic_info) => {
            let error_message = if let Some(s) = panic_info.downcast_ref::<&str>() {
                s.to_string()
            } else if let Some(s) = panic_info.downcast_ref::<String>() {
                s.clone()
            } else {
                "Unknown panic occurred".to_string()
            };
            error(error_message)
        }
    }
}

/// Safely executes a callback function that may return a Result type.
/// This is useful for functions that already return Result and you want to
/// additionally catch panics.
///
/// # Arguments
///
/// * `callback` - The function to execute safely
///
/// # Returns
///
/// A SafeResult containing either the successful value or the error
///
/// # Example
///
/// ```
/// use umt_rust::error::umt_safe_execute_result;
///
/// let result = umt_safe_execute_result(|| -> Result<i32, String> {
///     Ok(42)
/// });
/// assert!(result.is_success());
///
/// let result = umt_safe_execute_result(|| -> Result<i32, String> {
///     Err("custom error".to_string())
/// });
/// assert!(result.is_error());
/// ```
#[inline]
pub fn umt_safe_execute_result<V, E, F>(callback: F) -> SafeResult<V, String>
where
    F: FnOnce() -> Result<V, E> + std::panic::UnwindSafe,
    E: std::fmt::Display,
{
    match catch_unwind(callback) {
        Ok(Ok(value)) => success(value),
        Ok(Err(e)) => error(e.to_string()),
        Err(panic_info) => {
            let error_message = if let Some(s) = panic_info.downcast_ref::<&str>() {
                s.to_string()
            } else if let Some(s) = panic_info.downcast_ref::<String>() {
                s.clone()
            } else {
                "Unknown panic occurred".to_string()
            };
            error(error_message)
        }
    }
}

/// A version of safe_execute that works with closures that capture mutable state.
/// Uses AssertUnwindSafe to mark the closure as unwind-safe.
///
/// # Safety
///
/// The caller must ensure that any captured state is in a valid state after
/// a panic is caught.
///
/// # Example
///
/// ```
/// use umt_rust::error::umt_safe_execute_mut;
///
/// let mut counter = 0;
/// let result = umt_safe_execute_mut(|| {
///     counter += 1;
///     counter
/// });
/// assert!(result.is_success());
/// assert_eq!(result.value(), Some(&1));
/// ```
#[inline]
pub fn umt_safe_execute_mut<V, F>(callback: F) -> SafeResult<V, String>
where
    F: FnOnce() -> V,
{
    match catch_unwind(AssertUnwindSafe(callback)) {
        Ok(value) => success(value),
        Err(panic_info) => {
            let error_message = if let Some(s) = panic_info.downcast_ref::<&str>() {
                s.to_string()
            } else if let Some(s) = panic_info.downcast_ref::<String>() {
                s.clone()
            } else {
                "Unknown panic occurred".to_string()
            };
            error(error_message)
        }
    }
}
