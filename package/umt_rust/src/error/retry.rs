use std::thread;
use std::time::Duration;

/// Configuration options for retry behavior
#[derive(Debug, Clone)]
pub struct RetryOptions<F>
where
    F: Fn(&dyn std::error::Error) -> bool,
{
    /// Maximum number of retry attempts (default: 3)
    pub retries: usize,
    /// Delay between retries in milliseconds (default: 1000)
    pub delay_ms: u64,
    /// Function to determine if an error should trigger a retry (default: always retry)
    pub should_retry: F,
}

impl Default for RetryOptions<fn(&dyn std::error::Error) -> bool> {
    fn default() -> Self {
        RetryOptions {
            retries: 3,
            delay_ms: 1000,
            should_retry: |_| true,
        }
    }
}

/// Builder for creating RetryOptions with a fluent API
#[derive(Debug, Clone)]
pub struct RetryOptionsBuilder<F>
where
    F: Fn(&dyn std::error::Error) -> bool,
{
    retries: usize,
    delay_ms: u64,
    should_retry: F,
}

impl RetryOptionsBuilder<fn(&dyn std::error::Error) -> bool> {
    /// Creates a new builder with default options
    pub fn new() -> Self {
        RetryOptionsBuilder {
            retries: 3,
            delay_ms: 1000,
            should_retry: |_| true,
        }
    }
}

impl Default for RetryOptionsBuilder<fn(&dyn std::error::Error) -> bool> {
    fn default() -> Self {
        Self::new()
    }
}

impl<F> RetryOptionsBuilder<F>
where
    F: Fn(&dyn std::error::Error) -> bool,
{
    /// Sets the maximum number of retry attempts
    pub fn retries(mut self, retries: usize) -> Self {
        self.retries = retries;
        self
    }

    /// Sets the delay between retries in milliseconds
    pub fn delay_ms(mut self, delay_ms: u64) -> Self {
        self.delay_ms = delay_ms;
        self
    }

    /// Sets the function to determine if an error should trigger a retry
    pub fn should_retry<G>(self, should_retry: G) -> RetryOptionsBuilder<G>
    where
        G: Fn(&dyn std::error::Error) -> bool,
    {
        RetryOptionsBuilder {
            retries: self.retries,
            delay_ms: self.delay_ms,
            should_retry,
        }
    }

    /// Builds the RetryOptions
    pub fn build(self) -> RetryOptions<F> {
        RetryOptions {
            retries: self.retries,
            delay_ms: self.delay_ms,
            should_retry: self.should_retry,
        }
    }
}

/// A simple error wrapper for retry operations
#[derive(Debug)]
pub struct RetryError {
    message: String,
}

impl RetryError {
    pub fn new(message: impl Into<String>) -> Self {
        RetryError {
            message: message.into(),
        }
    }
}

impl std::fmt::Display for RetryError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.message)
    }
}

impl std::error::Error for RetryError {}

/// Retries a given function with configurable retry logic.
///
/// This function attempts to execute the provided operation multiple times
/// with a configurable delay between attempts and a predicate to determine
/// if a retry should be attempted.
///
/// # Type Parameters
///
/// * `T` - The return type of the function
/// * `E` - The error type (must implement std::error::Error)
/// * `O` - The operation function type
/// * `S` - The should_retry function type
///
/// # Arguments
///
/// * `operation` - The function to retry
/// * `options` - Configuration options for retry behavior
///
/// # Returns
///
/// Returns `Ok(T)` if the operation succeeds, or `Err(E)` with the final error
/// if all retry attempts fail
///
/// # Example
///
/// ```
/// use umt_rust::error::{umt_retry, RetryOptionsBuilder, RetryError};
///
/// let mut attempt = 0;
/// let result = umt_retry(
///     || -> Result<i32, RetryError> {
///         attempt += 1;
///         if attempt < 3 {
///             Err(RetryError::new("not yet"))
///         } else {
///             Ok(42)
///         }
///     },
///     RetryOptionsBuilder::new()
///         .retries(5)
///         .delay_ms(1)
///         .build(),
/// );
///
/// assert_eq!(result.unwrap(), 42);
/// ```
#[inline]
pub fn umt_retry<T, E, O, S>(mut operation: O, options: RetryOptions<S>) -> Result<T, E>
where
    E: std::error::Error,
    O: FnMut() -> Result<T, E>,
    S: Fn(&dyn std::error::Error) -> bool,
{
    let mut remaining_attempts = options.retries;

    loop {
        match operation() {
            Ok(value) => return Ok(value),
            Err(error) => {
                if remaining_attempts == 0 || !(options.should_retry)(&error) {
                    return Err(error);
                }
                remaining_attempts -= 1;
                thread::sleep(Duration::from_millis(options.delay_ms));
            }
        }
    }
}

/// Retries a given function with default options (3 retries, 1000ms delay).
///
/// # Example
///
/// ```
/// use umt_rust::error::{umt_retry_default, RetryError};
///
/// let mut attempt = 0;
/// let result = umt_retry_default(|| -> Result<i32, RetryError> {
///     attempt += 1;
///     if attempt < 2 {
///         Err(RetryError::new("first failure"))
///     } else {
///         Ok(42)
///     }
/// });
///
/// assert_eq!(result.unwrap(), 42);
/// ```
#[inline]
pub fn umt_retry_default<T, E, O>(operation: O) -> Result<T, E>
where
    E: std::error::Error,
    O: FnMut() -> Result<T, E>,
{
    umt_retry(operation, RetryOptions::default())
}

/// Retries a given function with simple configuration (retries count and delay).
///
/// # Arguments
///
/// * `operation` - The function to retry
/// * `retries` - Maximum number of retry attempts
/// * `delay_ms` - Delay between retries in milliseconds
///
/// # Example
///
/// ```
/// use umt_rust::error::{umt_retry_simple, RetryError};
///
/// let mut attempt = 0;
/// let result = umt_retry_simple(
///     || -> Result<i32, RetryError> {
///         attempt += 1;
///         if attempt < 2 {
///             Err(RetryError::new("temporary failure"))
///         } else {
///             Ok(42)
///         }
///     },
///     3,
///     1,
/// );
///
/// assert_eq!(result.unwrap(), 42);
/// ```
#[inline]
pub fn umt_retry_simple<T, E, O>(operation: O, retries: usize, delay_ms: u64) -> Result<T, E>
where
    E: std::error::Error,
    O: FnMut() -> Result<T, E>,
{
    umt_retry(
        operation,
        RetryOptionsBuilder::new()
            .retries(retries)
            .delay_ms(delay_ms)
            .build(),
    )
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::cell::Cell;
    use std::rc::Rc;

    #[test]
    fn test_retry_success_on_first_attempt() {
        let call_count = Rc::new(Cell::new(0));
        let call_count_clone = call_count.clone();

        let result = umt_retry(
            || -> Result<&str, RetryError> {
                call_count_clone.set(call_count_clone.get() + 1);
                Ok("success")
            },
            RetryOptionsBuilder::new().delay_ms(1).build(),
        );

        assert_eq!(result.unwrap(), "success");
        assert_eq!(call_count.get(), 1);
    }

    #[test]
    fn test_retry_different_value_types() {
        // Number
        let result = umt_retry(
            || -> Result<i32, RetryError> { Ok(42) },
            RetryOptionsBuilder::new().delay_ms(1).build(),
        );
        assert_eq!(result.unwrap(), 42);

        // Vector
        let result = umt_retry(
            || -> Result<Vec<i32>, RetryError> { Ok(vec![1, 2, 3]) },
            RetryOptionsBuilder::new().delay_ms(1).build(),
        );
        assert_eq!(result.unwrap(), vec![1, 2, 3]);
    }

    #[test]
    fn test_retry_fails_after_max_retries() {
        let call_count = Rc::new(Cell::new(0));
        let call_count_clone = call_count.clone();

        let result = umt_retry(
            || -> Result<i32, RetryError> {
                call_count_clone.set(call_count_clone.get() + 1);
                Err(RetryError::new("test error"))
            },
            RetryOptionsBuilder::new().retries(3).delay_ms(1).build(),
        );

        assert!(result.is_err());
        assert_eq!(result.unwrap_err().to_string(), "test error");
        assert_eq!(call_count.get(), 4); // 1 initial + 3 retries
    }

    #[test]
    fn test_retry_succeeds_on_second_attempt() {
        let call_count = Rc::new(Cell::new(0));
        let call_count_clone = call_count.clone();

        let result = umt_retry(
            || -> Result<&str, RetryError> {
                call_count_clone.set(call_count_clone.get() + 1);
                if call_count_clone.get() < 2 {
                    Err(RetryError::new("first failure"))
                } else {
                    Ok("success")
                }
            },
            RetryOptionsBuilder::new().retries(3).delay_ms(1).build(),
        );

        assert_eq!(result.unwrap(), "success");
        assert_eq!(call_count.get(), 2);
    }

    #[test]
    fn test_retry_succeeds_on_last_attempt() {
        let call_count = Rc::new(Cell::new(0));
        let call_count_clone = call_count.clone();

        let result = umt_retry(
            || -> Result<&str, RetryError> {
                call_count_clone.set(call_count_clone.get() + 1);
                if call_count_clone.get() < 4 {
                    Err(RetryError::new(format!(
                        "failure {}",
                        call_count_clone.get()
                    )))
                } else {
                    Ok("success")
                }
            },
            RetryOptionsBuilder::new().retries(3).delay_ms(1).build(),
        );

        assert_eq!(result.unwrap(), "success");
        assert_eq!(call_count.get(), 4);
    }

    #[test]
    fn test_retry_with_zero_retries() {
        let call_count = Rc::new(Cell::new(0));
        let call_count_clone = call_count.clone();

        let result = umt_retry(
            || -> Result<i32, RetryError> {
                call_count_clone.set(call_count_clone.get() + 1);
                Err(RetryError::new("test error"))
            },
            RetryOptionsBuilder::new().retries(0).delay_ms(1).build(),
        );

        assert!(result.is_err());
        assert_eq!(call_count.get(), 1);
    }

    #[test]
    fn test_retry_with_one_retry() {
        let call_count = Rc::new(Cell::new(0));
        let call_count_clone = call_count.clone();

        let result = umt_retry(
            || -> Result<i32, RetryError> {
                call_count_clone.set(call_count_clone.get() + 1);
                Err(RetryError::new("test error"))
            },
            RetryOptionsBuilder::new().retries(1).delay_ms(1).build(),
        );

        assert!(result.is_err());
        assert_eq!(call_count.get(), 2); // 1 initial + 1 retry
    }

    #[test]
    fn test_should_retry_returns_false() {
        let call_count = Rc::new(Cell::new(0));
        let call_count_clone = call_count.clone();

        let result = umt_retry(
            || -> Result<i32, RetryError> {
                call_count_clone.set(call_count_clone.get() + 1);
                Err(RetryError::new("test error"))
            },
            RetryOptionsBuilder::new()
                .retries(3)
                .delay_ms(1)
                .should_retry(|_| false)
                .build(),
        );

        assert!(result.is_err());
        assert_eq!(call_count.get(), 1);
    }

    #[test]
    fn test_should_retry_for_specific_errors() {
        let call_count = Rc::new(Cell::new(0));
        let call_count_clone = call_count.clone();

        let result = umt_retry(
            || -> Result<i32, RetryError> {
                call_count_clone.set(call_count_clone.get() + 1);
                if call_count_clone.get() == 1 {
                    Err(RetryError::new("Network error"))
                } else {
                    Err(RetryError::new("Validation error"))
                }
            },
            RetryOptionsBuilder::new()
                .retries(3)
                .delay_ms(1)
                .should_retry(|error| error.to_string().contains("Network"))
                .build(),
        );

        assert!(result.is_err());
        assert_eq!(result.unwrap_err().to_string(), "Validation error");
        assert_eq!(call_count.get(), 2);
    }

    #[test]
    fn test_default_should_retry_always_true() {
        let call_count = Rc::new(Cell::new(0));
        let call_count_clone = call_count.clone();

        let result = umt_retry(
            || -> Result<i32, RetryError> {
                call_count_clone.set(call_count_clone.get() + 1);
                Err(RetryError::new("test error"))
            },
            RetryOptionsBuilder::new().retries(2).delay_ms(1).build(),
        );

        assert!(result.is_err());
        assert_eq!(call_count.get(), 3); // 1 initial + 2 retries
    }

    #[test]
    fn test_retry_default_function() {
        let call_count = Rc::new(Cell::new(0));
        let call_count_clone = call_count.clone();

        let result = umt_retry_default(|| -> Result<i32, RetryError> {
            call_count_clone.set(call_count_clone.get() + 1);
            if call_count_clone.get() < 2 {
                Err(RetryError::new("temporary"))
            } else {
                Ok(42)
            }
        });

        assert_eq!(result.unwrap(), 42);
        assert_eq!(call_count.get(), 2);
    }

    #[test]
    fn test_retry_simple_function() {
        let call_count = Rc::new(Cell::new(0));
        let call_count_clone = call_count.clone();

        let result = umt_retry_simple(
            || -> Result<i32, RetryError> {
                call_count_clone.set(call_count_clone.get() + 1);
                if call_count_clone.get() < 2 {
                    Err(RetryError::new("temporary"))
                } else {
                    Ok(42)
                }
            },
            3,
            1,
        );

        assert_eq!(result.unwrap(), 42);
        assert_eq!(call_count.get(), 2);
    }

    #[test]
    fn test_mixed_success_and_failure() {
        let call_count = Rc::new(Cell::new(0));
        let call_count_clone = call_count.clone();

        let result = umt_retry(
            || -> Result<&str, RetryError> {
                call_count_clone.set(call_count_clone.get() + 1);
                if call_count_clone.get() == 1 {
                    Err(RetryError::new("temp failure"))
                } else {
                    Ok("temp success")
                }
            },
            RetryOptionsBuilder::new().retries(3).delay_ms(1).build(),
        );

        assert_eq!(result.unwrap(), "temp success");
        assert_eq!(call_count.get(), 2);
    }
}
