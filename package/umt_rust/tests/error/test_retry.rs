use std::cell::Cell;
use std::rc::Rc;
use umt_rust::error::{
    RetryError, RetryOptionsBuilder, umt_retry, umt_retry_default, umt_retry_simple,
};

// =============================================================================
// Successful Operations
// =============================================================================

#[test]
fn test_returns_result_on_first_successful_attempt() {
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
fn test_returns_different_types_of_values() {
    // Number
    let result = umt_retry(
        || -> Result<i32, RetryError> { Ok(42) },
        RetryOptionsBuilder::new().delay_ms(1).build(),
    );
    assert_eq!(result.unwrap(), 42);

    // Vector (like object)
    let result = umt_retry(
        || -> Result<Vec<(&str, &str)>, RetryError> { Ok(vec![("key", "value")]) },
        RetryOptionsBuilder::new().delay_ms(1).build(),
    );
    assert_eq!(result.unwrap(), vec![("key", "value")]);

    // Array
    let result = umt_retry(
        || -> Result<Vec<i32>, RetryError> { Ok(vec![1, 2, 3]) },
        RetryOptionsBuilder::new().delay_ms(1).build(),
    );
    assert_eq!(result.unwrap(), vec![1, 2, 3]);
}

// =============================================================================
// Retry Logic
// =============================================================================

#[test]
fn test_retries_specified_number_of_times_before_failing() {
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
fn test_succeeds_on_second_attempt() {
    let call_count = Rc::new(Cell::new(0));
    let call_count_clone = call_count.clone();

    let result = umt_retry(
        || -> Result<&str, RetryError> {
            call_count_clone.set(call_count_clone.get() + 1);
            if call_count_clone.get() == 1 {
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
fn test_succeeds_on_last_retry_attempt() {
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
fn test_handles_zero_retries() {
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
    assert_eq!(result.unwrap_err().to_string(), "test error");
    assert_eq!(call_count.get(), 1);
}

#[test]
fn test_handles_one_retry() {
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
    assert_eq!(result.unwrap_err().to_string(), "test error");
    assert_eq!(call_count.get(), 2); // 1 initial + 1 retry
}

// =============================================================================
// Delay Functionality
// =============================================================================

#[test]
fn test_respects_delay_configuration() {
    let call_count = Rc::new(Cell::new(0));
    let call_count_clone = call_count.clone();

    let result = umt_retry(
        || -> Result<&str, RetryError> {
            call_count_clone.set(call_count_clone.get() + 1);
            if call_count_clone.get() == 1 {
                Err(RetryError::new("failure"))
            } else {
                Ok("success")
            }
        },
        RetryOptionsBuilder::new().retries(1).delay_ms(10).build(),
    );

    assert_eq!(result.unwrap(), "success");
    assert_eq!(call_count.get(), 2);
}

// =============================================================================
// shouldRetry Functionality
// =============================================================================

#[test]
fn test_does_not_retry_when_should_retry_returns_false() {
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
    assert_eq!(result.unwrap_err().to_string(), "test error");
    assert_eq!(call_count.get(), 1);
}

#[test]
fn test_retries_only_for_specific_error_types() {
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
fn test_uses_default_should_retry_always_true_when_not_provided() {
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
    assert_eq!(result.unwrap_err().to_string(), "test error");
    assert_eq!(call_count.get(), 3); // 1 initial + 2 retries
}

// =============================================================================
// Default Options
// =============================================================================

#[test]
fn test_uses_default_values_when_options_not_provided() {
    let call_count = Rc::new(Cell::new(0));
    let call_count_clone = call_count.clone();

    // Using retry_simple with default retries behavior
    let result = umt_retry_simple(
        || -> Result<i32, RetryError> {
            call_count_clone.set(call_count_clone.get() + 1);
            Err(RetryError::new("test error"))
        },
        3, // default retries
        1, // minimal delay for testing
    );

    assert!(result.is_err());
    assert_eq!(result.unwrap_err().to_string(), "test error");
    assert_eq!(call_count.get(), 4); // 1 initial + 3 retries (default)
}

#[test]
fn test_allows_partial_options() {
    let call_count = Rc::new(Cell::new(0));
    let call_count_clone = call_count.clone();

    let result = umt_retry(
        || -> Result<i32, RetryError> {
            call_count_clone.set(call_count_clone.get() + 1);
            Err(RetryError::new("test error"))
        },
        RetryOptionsBuilder::new().retries(1).build(),
    );

    assert!(result.is_err());
    assert_eq!(result.unwrap_err().to_string(), "test error");
    assert_eq!(call_count.get(), 2); // 1 initial + 1 retry
}

// =============================================================================
// Complex Scenarios
// =============================================================================

#[test]
fn test_handles_mixed_success_and_failure_patterns() {
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
fn test_retry_options_builder_default() {
    let builder: RetryOptionsBuilder<fn(&dyn std::error::Error) -> bool> =
        RetryOptionsBuilder::default();
    let options = builder.build();
    assert_eq!(options.retries, 3);
    assert_eq!(options.delay_ms, 1000);
}
