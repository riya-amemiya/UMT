use umt_rust::error::{ErrorType, SafeResult, SuccessType, umt_unwrap_or};

#[test]
fn test_unwrap_or_returns_success_value() {
    let success: SafeResult<i32, String> = SafeResult::Success(SuccessType { value: 5 });
    assert_eq!(umt_unwrap_or(success, 0), 5);
}

#[test]
fn test_unwrap_or_returns_default_for_error() {
    let error: SafeResult<i32, String> = SafeResult::Error(ErrorType {
        error: "fail".to_string(),
    });
    assert_eq!(umt_unwrap_or(error, 0), 0);
}

#[test]
fn test_unwrap_or_keeps_none_success_value() {
    let success: SafeResult<Option<i32>, String> = SafeResult::Success(SuccessType { value: None });
    assert_eq!(umt_unwrap_or(success, Some(7)), None);
}
