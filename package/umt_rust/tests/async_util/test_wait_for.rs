use std::cell::Cell;
use umt_rust::async_util::{WaitForError, WaitForOptions, umt_wait_for};

#[test]
fn test_wait_for_resolves_immediately_when_initially_truthy() {
    let result = umt_wait_for(
        || Ok::<Option<i32>, String>(Some(42)),
        WaitForOptions::default(),
    );
    assert_eq!(result, Ok(42));
}

#[test]
fn test_wait_for_resolves_once_condition_becomes_truthy() {
    let counter = Cell::new(0);
    let result = umt_wait_for(
        || {
            counter.set(counter.get() + 1);
            Ok::<Option<&str>, String>(if counter.get() >= 3 {
                Some("done")
            } else {
                None
            })
        },
        WaitForOptions {
            timeout: 500,
            interval: 1,
        },
    );
    assert_eq!(result, Ok("done"));
}

#[test]
fn test_wait_for_rejects_on_timeout() {
    let result: Result<i32, WaitForError<String>> = umt_wait_for(
        || Ok::<Option<i32>, String>(None),
        WaitForOptions {
            timeout: 20,
            interval: 5,
        },
    );
    assert_eq!(result, Err(WaitForError::TimedOut { ms: 20 }));
}

#[test]
fn test_wait_for_propagates_condition_error() {
    let result: Result<i32, WaitForError<String>> = umt_wait_for(
        || Err::<Option<i32>, String>("boom".to_string()),
        WaitForOptions {
            timeout: 100,
            interval: 5,
        },
    );
    assert_eq!(result, Err(WaitForError::Condition("boom".to_string())));
}

#[test]
fn test_wait_for_timeout_message_format() {
    let error: WaitForError<String> = WaitForError::TimedOut { ms: 20 };
    assert_eq!(error.to_string(), "waitFor timed out after 20ms");
}
