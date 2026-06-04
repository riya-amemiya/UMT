use umt_rust::error::umt_success_function;

#[test]
fn test_success_function_creates_success_result() {
    let result = umt_success_function(42);
    assert_eq!(result.value, 42);
}

#[test]
fn test_success_function_with_string_value() {
    let result = umt_success_function("test".to_string());
    assert_eq!(result.value, "test".to_string());
}

#[test]
fn test_success_function_with_object_value() {
    let result = umt_success_function(vec![1, 2, 3]);
    assert_eq!(result.value, vec![1, 2, 3]);
}
