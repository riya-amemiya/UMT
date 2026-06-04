//! Tests for union function

use umt_rust::validate::core::ValidateCoreReturnType;
use umt_rust::validate::object::umt_union;

fn negative(value: &i32) -> ValidateCoreReturnType<i32> {
    ValidateCoreReturnType {
        validate: *value < 0,
        message: "not negative".to_string(),
        type_value: *value,
    }
}

fn greater_than_ten(value: &i32) -> ValidateCoreReturnType<i32> {
    ValidateCoreReturnType {
        validate: *value > 10,
        message: "not greater than ten".to_string(),
        type_value: *value,
    }
}

fn members() -> Vec<Box<dyn Fn(&i32) -> ValidateCoreReturnType<i32>>> {
    vec![Box::new(negative), Box::new(greater_than_ten)]
}

#[test]
fn test_union_accepts_values_matching_the_first_validator() {
    let validator = umt_union(members());
    let result = validator(&-1);
    assert!(result.validate);
    assert_eq!(result.message, "");
}

#[test]
fn test_union_accepts_values_matching_the_second_validator() {
    let validator = umt_union(members());
    let result = validator(&42);
    assert!(result.validate);
    assert_eq!(result.message, "");
}

#[test]
fn test_union_rejects_values_matching_none_of_the_validators() {
    let validator = umt_union(members());
    let result = validator(&5);
    assert!(!result.validate);
}

#[test]
fn test_union_returns_the_last_failing_validators_message() {
    let validator = umt_union(members());
    let result = validator(&5);
    assert!(!result.validate);
    assert_eq!(result.message, "not greater than ten");
}

#[test]
fn test_union_works_with_three_or_more_validators() {
    let validator = umt_union(vec![
        Box::new(negative) as Box<dyn Fn(&i32) -> ValidateCoreReturnType<i32>>,
        Box::new(greater_than_ten),
        Box::new(|value: &i32| ValidateCoreReturnType {
            validate: *value == 0,
            message: "not zero".to_string(),
            type_value: *value,
        }),
    ]);
    assert!(validator(&-1).validate);
    assert!(validator(&42).validate);
    assert!(validator(&0).validate);
    assert!(!validator(&5).validate);
}

#[test]
fn test_union_preserves_the_original_value_in_the_type_field() {
    let validator = umt_union(members());
    let ok = validator(&-3);
    assert_eq!(ok.type_value, -3);
    let bad = validator(&5);
    assert_eq!(bad.type_value, 5);
}

#[test]
fn test_union_works_with_string_validators() {
    let validator = umt_union(vec![
        Box::new(|value: &String| ValidateCoreReturnType {
            validate: value.is_empty(),
            message: "not empty".to_string(),
            type_value: value.clone(),
        }) as Box<dyn Fn(&String) -> ValidateCoreReturnType<String>>,
        Box::new(|value: &String| ValidateCoreReturnType {
            validate: value.len() > 3,
            message: "too short".to_string(),
            type_value: value.clone(),
        }),
    ]);
    assert!(validator(&String::new()).validate);
    assert!(validator(&"hello".to_string()).validate);
    let bad = validator(&"ab".to_string());
    assert!(!bad.validate);
    assert_eq!(bad.message, "too short");
}
