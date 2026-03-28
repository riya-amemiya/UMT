use umt_rust::error::{umt_map_result, ErrorType, SafeResult, SuccessType};

#[test]
fn test_map_result_success_transforms_value() {
    let success: SafeResult<i32, String> = SafeResult::Success(SuccessType { value: 5 });
    let result = umt_map_result(success, |n| n * 2);
    assert!(result.is_success());
    assert_eq!(result.value(), Some(&10));
}

#[test]
fn test_map_result_success_changes_type() {
    let success: SafeResult<i32, String> = SafeResult::Success(SuccessType { value: 42 });
    let result = umt_map_result(success, |n| format!("value: {}", n));
    assert!(result.is_success());
    assert_eq!(result.value(), Some(&"value: 42".to_string()));
}

#[test]
fn test_map_result_error_returns_unchanged() {
    let error: SafeResult<i32, String> =
        SafeResult::Error(ErrorType {
            error: "fail".to_string(),
        });
    let result = umt_map_result(error, |n| n * 2);
    assert!(result.is_error());
    assert_eq!(result.error(), Some(&"fail".to_string()));
}

#[test]
fn test_map_result_with_vec() {
    let success: SafeResult<Vec<i32>, String> =
        SafeResult::Success(SuccessType {
            value: vec![1, 2, 3],
        });
    let result = umt_map_result(success, |v| v.len());
    assert!(result.is_success());
    assert_eq!(result.value(), Some(&3));
}
