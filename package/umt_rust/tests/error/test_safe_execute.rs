use umt_rust::error::{umt_safe_execute, umt_safe_execute_mut, umt_safe_execute_result};

#[test]
fn test_successful_operations_with_string() {
    let result = umt_safe_execute(|| "test".to_string());
    assert!(result.is_success());
    assert_eq!(result.result_type(), "success");
    assert_eq!(result.value(), Some(&"test".to_string()));
}

#[test]
fn test_successful_operations_with_number() {
    let result = umt_safe_execute(|| 42);
    assert!(result.is_success());
    assert_eq!(result.value(), Some(&42));
}

#[test]
fn test_successful_operations_with_object() {
    #[derive(Debug, Clone, PartialEq)]
    struct TestObject {
        key: String,
    }

    let test_object = TestObject {
        key: "value".to_string(),
    };
    let expected = test_object.clone();
    let result = umt_safe_execute(move || test_object);

    assert!(result.is_success());
    assert_eq!(result.value(), Some(&expected));
}

#[test]
fn test_error_handling_with_panic() {
    let result = umt_safe_execute(|| -> i32 { panic!("test error") });

    assert!(result.is_error());
    assert_eq!(result.result_type(), "error");
    assert_eq!(result.error(), Some(&"test error".to_string()));
}

#[test]
fn test_error_handling_with_custom_error_message() {
    let result = umt_safe_execute(|| -> i32 { panic!("custom type error") });

    assert!(result.is_error());
    assert_eq!(result.error(), Some(&"custom type error".to_string()));
}

#[test]
fn test_safe_execute_result_with_ok() {
    let result = umt_safe_execute_result(|| -> Result<i32, String> { Ok(42) });

    assert!(result.is_success());
    assert_eq!(result.value(), Some(&42));
}

#[test]
fn test_safe_execute_result_with_err() {
    let result =
        umt_safe_execute_result(|| -> Result<i32, String> { Err("custom error".to_string()) });

    assert!(result.is_error());
    assert_eq!(result.error(), Some(&"custom error".to_string()));
}

#[test]
fn test_safe_execute_mut_with_mutable_state() {
    let mut counter = 0;
    let result = umt_safe_execute_mut(|| {
        counter += 1;
        counter
    });

    assert!(result.is_success());
    assert_eq!(result.value(), Some(&1));
    assert_eq!(counter, 1);
}

#[test]
fn test_safe_result_to_result_conversion() {
    let success = umt_safe_execute(|| 42);
    assert_eq!(success.to_result(), Ok(42));

    let error = umt_safe_execute(|| -> i32 { panic!("error") });
    assert!(error.to_result().is_err());
}

#[test]
fn test_safe_execute_with_complex_computation() {
    let result = umt_safe_execute(|| {
        let mut sum = 0;
        for i in 1..=100 {
            sum += i;
        }
        sum
    });

    assert!(result.is_success());
    assert_eq!(result.value(), Some(&5050));
}

#[test]
fn test_safe_execute_with_vec() {
    let result = umt_safe_execute(|| vec![1, 2, 3, 4, 5]);

    assert!(result.is_success());
    assert_eq!(result.value(), Some(&vec![1, 2, 3, 4, 5]));
}
