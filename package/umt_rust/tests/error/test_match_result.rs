use umt_rust::error::{ErrorType, SafeResult, SuccessType, umt_match_result};

#[test]
fn test_match_result_success_calls_on_success() {
    let result: SafeResult<i32, String> = SafeResult::Success(SuccessType { value: 42 });
    let output = umt_match_result(
        result,
        |v| format!("Got {}", v),
        |e| format!("Failed: {}", e),
    );
    assert_eq!(output, "Got 42");
}

#[test]
fn test_match_result_error_calls_on_error() {
    let result: SafeResult<i32, String> = SafeResult::Error(ErrorType {
        error: "oops".to_string(),
    });
    let output = umt_match_result(
        result,
        |v| format!("Got {}", v),
        |e| format!("Failed: {}", e),
    );
    assert_eq!(output, "Failed: oops");
}

#[test]
fn test_match_result_returns_different_type() {
    let result: SafeResult<i32, String> = SafeResult::Success(SuccessType { value: 10 });
    let output = umt_match_result(result, |v| v * 2, |_e| -1);
    assert_eq!(output, 20);
}

#[test]
fn test_match_result_error_returns_different_type() {
    let result: SafeResult<i32, String> = SafeResult::Error(ErrorType {
        error: "fail".to_string(),
    });
    let output = umt_match_result(result, |v| v * 2, |_e| -1);
    assert_eq!(output, -1);
}
