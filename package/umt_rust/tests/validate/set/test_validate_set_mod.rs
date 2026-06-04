use std::collections::HashSet;

use umt_rust::validate::core::ValidateCoreReturnType;
use umt_rust::validate::set::*;

fn ok<T: Clone>(value: &T) -> ValidateCoreReturnType<T> {
    ValidateCoreReturnType {
        validate: true,
        message: String::new(),
        type_value: value.clone(),
    }
}

#[test]
fn test_validate_set_no_validator() {
    let set: HashSet<i32> = HashSet::new();
    let result = umt_validate_set(&set, None::<fn(&i32) -> ValidateCoreReturnType<i32>>);
    assert!(result.validate);

    let set: HashSet<i32> = [1, 2].into_iter().collect();
    let result = umt_validate_set(&set, None::<fn(&i32) -> ValidateCoreReturnType<i32>>);
    assert!(result.validate);
}

#[test]
fn test_validate_set_with_validator() {
    let set: HashSet<i32> = [2, 4, 6].into_iter().collect();
    let result = umt_validate_set(&set, Some(ok::<i32>));
    assert!(result.validate);
}

#[test]
fn test_validate_set_propagates_item_message() {
    let set: HashSet<i32> = [1].into_iter().collect();
    let result = umt_validate_set(
        &set,
        Some(|value: &i32| ValidateCoreReturnType {
            validate: false,
            message: "must be string".to_string(),
            type_value: *value,
        }),
    );
    assert!(!result.validate);
    assert_eq!(result.message, "must be string");
}

#[test]
fn test_validate_set_empty_with_validator() {
    let set: HashSet<i32> = HashSet::new();
    let result = umt_validate_set(
        &set,
        Some(|value: &i32| ValidateCoreReturnType {
            validate: false,
            message: "should not run".to_string(),
            type_value: *value,
        }),
    );
    assert!(result.validate);
}

#[test]
fn test_validate_set_returns_type_value() {
    let set: HashSet<i32> = [1, 2, 3].into_iter().collect();
    let result = umt_validate_set(&set, None::<fn(&i32) -> ValidateCoreReturnType<i32>>);
    assert_eq!(result.type_value, set);
}

#[test]
fn test_set_validator_success() {
    let validator = umt_set_validator(|value: &i32| ValidateCoreReturnType {
        validate: *value > 0,
        message: String::new(),
        type_value: *value,
    });
    let set: HashSet<i32> = [1, 2, 3].into_iter().collect();
    let result = validator(&set);
    assert!(result.validate);
    assert_eq!(result.message, "");
}

#[test]
fn test_set_validator_failure_propagates_message() {
    let validator = umt_set_validator(|value: &i32| ValidateCoreReturnType {
        validate: *value > 0,
        message: "must be positive".to_string(),
        type_value: *value,
    });
    let set: HashSet<i32> = [-1].into_iter().collect();
    let result = validator(&set);
    assert!(!result.validate);
    assert_eq!(result.message, "must be positive");
}

#[test]
fn test_set_validator_empty_set() {
    let validator = umt_set_validator(|value: &i32| ValidateCoreReturnType {
        validate: false,
        message: "should not run".to_string(),
        type_value: *value,
    });
    let set: HashSet<i32> = HashSet::new();
    let result = validator(&set);
    assert!(result.validate);
}

#[test]
fn test_set_validator_with_strings() {
    let validator = umt_set_validator(|value: &String| ValidateCoreReturnType {
        validate: !value.is_empty(),
        message: "must not be empty".to_string(),
        type_value: value.clone(),
    });
    let set: HashSet<String> = ["a".to_string(), "b".to_string()].into_iter().collect();
    let result = validator(&set);
    assert!(result.validate);
}
