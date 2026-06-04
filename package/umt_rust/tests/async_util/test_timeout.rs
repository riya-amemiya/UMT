use std::thread;
use std::time::Duration;
use umt_rust::async_util::{TimeoutError, umt_timeout};

#[test]
fn test_timeout_resolves_when_operation_completes_in_time() {
    let result = umt_timeout(
        || {
            thread::sleep(Duration::from_millis(10));
            Ok::<&str, String>("done")
        },
        1000,
    );
    assert_eq!(result, Ok("done"));
}

#[test]
fn test_timeout_rejects_when_operation_exceeds_time() {
    let result = umt_timeout(
        || {
            thread::sleep(Duration::from_millis(200));
            Ok::<&str, String>("done")
        },
        50,
    );
    assert_eq!(result, Err(TimeoutError::TimedOut { ms: 50 }));
}

#[test]
fn test_timeout_rejects_with_original_error() {
    let result: Result<&str, TimeoutError<String>> =
        umt_timeout(|| Err("original error".to_string()), 1000);
    assert_eq!(
        result,
        Err(TimeoutError::Operation("original error".to_string()))
    );
}

#[test]
fn test_timeout_error_message_format() {
    let error: TimeoutError<String> = TimeoutError::TimedOut { ms: 50 };
    assert_eq!(error.to_string(), "Timed out after 50ms");
}
