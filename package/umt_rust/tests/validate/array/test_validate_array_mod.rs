use umt_rust::validate::array::*;

#[test]
fn test_validate_array_no_validator() {
    let arr = vec![1, 2, 3];
    let result = umt_validate_array(&arr, None::<fn(&i32) -> bool>, None);
    assert!(result.validate);
}

#[test]
fn test_validate_array_with_validator() {
    let arr = vec![2, 4, 6];
    let result = umt_validate_array(&arr, Some(|x: &i32| x % 2 == 0), None);
    assert!(result.validate);
}

#[test]
fn test_validate_array_fails() {
    let arr = vec![1, 2, 3];
    let result = umt_validate_array(&arr, Some(|x: &i32| x % 2 == 0), Some("must be even"));
    assert!(!result.validate);
    assert_eq!(result.message, "must be even");
}

#[test]
fn test_validate_array_fails_without_message() {
    let arr = vec![1, 2, 3];
    let result = umt_validate_array(&arr, Some(|x: &i32| x % 2 == 0), None);
    assert!(!result.validate);
    assert_eq!(result.message, "");
}

#[test]
fn test_array_validator_success() {
    let validator = umt_array_validator(|x: &i32| *x > 0, None);
    let arr = vec![1, 2, 3];
    let result = validator(&arr);
    assert!(result.validate);
    assert_eq!(result.message, "");
}

#[test]
fn test_array_validator_failure() {
    let validator = umt_array_validator(|x: &i32| *x > 0, Some("must be positive".to_string()));
    let arr = vec![1, -2, 3];
    let result = validator(&arr);
    assert!(!result.validate);
    assert_eq!(result.message, "must be positive");
}

#[test]
fn test_array_validator_failure_without_message() {
    let validator = umt_array_validator(|x: &i32| *x > 0, None);
    let arr = vec![1, -2, 3];
    let result = validator(&arr);
    assert!(!result.validate);
    assert_eq!(result.message, "");
}

#[test]
fn test_array_validator_empty_array() {
    let validator = umt_array_validator(|x: &i32| *x > 0, None);
    let arr: Vec<i32> = vec![];
    let result = validator(&arr);
    assert!(result.validate);
}

#[test]
fn test_validate_array_empty() {
    let arr: Vec<i32> = vec![];
    let result = umt_validate_array(&arr, Some(|x: &i32| *x > 0), None);
    assert!(result.validate);
}

#[test]
fn test_validate_array_returns_type_value() {
    let arr = vec![1, 2, 3];
    let result = umt_validate_array(&arr, None::<fn(&i32) -> bool>, None);
    assert_eq!(result.type_value, vec![1, 2, 3]);
}
