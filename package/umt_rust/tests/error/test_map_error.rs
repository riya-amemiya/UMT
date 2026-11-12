use std::cell::Cell;
use umt_rust::error::{ErrorType, SafeResult, SuccessType, umt_map_error};

#[test]
fn test_map_error_transforms_error() {
    let error: SafeResult<i32, String> = SafeResult::Error(ErrorType {
        error: "fail".to_string(),
    });
    let result = umt_map_error(error, |message| format!("wrapped: {}", message));
    assert_eq!(
        result,
        SafeResult::Error(ErrorType {
            error: "wrapped: fail".to_string()
        })
    );
}

#[test]
fn test_map_error_returns_success_unchanged() {
    let calls = Cell::new(0);
    let success: SafeResult<i32, String> = SafeResult::Success(SuccessType { value: 5 });
    let result = umt_map_error(success, |message| {
        calls.set(calls.get() + 1);
        message.len()
    });
    assert_eq!(result, SafeResult::Success(SuccessType { value: 5 }));
    assert_eq!(calls.get(), 0);
}

#[test]
fn test_map_error_changes_error_type() {
    let error: SafeResult<i32, &str> = SafeResult::Error(ErrorType { error: "boom" });
    let result = umt_map_error(error, |message| message.len());
    assert_eq!(result.error(), Some(&4));
}
