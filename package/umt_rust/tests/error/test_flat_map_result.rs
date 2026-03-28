use umt_rust::error::{ErrorType, SafeResult, SuccessType, umt_flat_map_result};

#[test]
fn test_flat_map_result_success_returns_new_success() {
    let success: SafeResult<i32, String> = SafeResult::Success(SuccessType { value: 5 });
    let result: SafeResult<i32, String> = umt_flat_map_result(success, |n| {
        SafeResult::Success(SuccessType { value: n * 2 })
    });
    assert!(result.is_success());
    assert_eq!(result.value(), Some(&10));
}

#[test]
fn test_flat_map_result_success_returns_new_error() {
    let success: SafeResult<i32, String> = SafeResult::Success(SuccessType { value: -1 });
    let result: SafeResult<i32, String> = umt_flat_map_result(success, |n| {
        if n > 0 {
            SafeResult::Success(SuccessType { value: n * 2 })
        } else {
            SafeResult::Error(ErrorType {
                error: "negative".to_string(),
            })
        }
    });
    assert!(result.is_error());
    assert_eq!(result.error(), Some(&"negative".to_string()));
}

#[test]
fn test_flat_map_result_error_returns_unchanged() {
    let error: SafeResult<i32, String> = SafeResult::Error(ErrorType {
        error: "original error".to_string(),
    });
    let result: SafeResult<i32, String> =
        umt_flat_map_result(error, |n| SafeResult::Success(SuccessType { value: n * 2 }));
    assert!(result.is_error());
    assert_eq!(result.error(), Some(&"original error".to_string()));
}

#[test]
fn test_flat_map_result_chaining() {
    let success: SafeResult<i32, String> = SafeResult::Success(SuccessType { value: 10 });
    let result: SafeResult<String, String> = umt_flat_map_result(success, |n| {
        SafeResult::Success(SuccessType {
            value: format!("result: {}", n),
        })
    });
    assert!(result.is_success());
    assert_eq!(result.value(), Some(&"result: 10".to_string()));
}
