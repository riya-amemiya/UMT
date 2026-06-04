use umt_rust::error::umt_error_function;

#[test]
fn test_error_function_creates_error_result() {
    let result = umt_error_function("test".to_string());
    assert_eq!(result.error, "test".to_string());
}

#[test]
fn test_error_function_with_string_slice() {
    let result = umt_error_function("string error");
    assert_eq!(result.error, "string error");
}

#[test]
fn test_error_function_with_numeric_error() {
    let result = umt_error_function(404);
    assert_eq!(result.error, 404);
}
