//! Tests for intersection function

use umt_rust::validate::core::ValidateCoreReturnType;
use umt_rust::validate::object::umt_intersection;

fn positive(value: &i32) -> ValidateCoreReturnType<i32> {
    ValidateCoreReturnType {
        validate: *value > 0,
        message: "not positive".to_string(),
        type_value: *value,
    }
}

fn less_than_hundred(value: &i32) -> ValidateCoreReturnType<i32> {
    ValidateCoreReturnType {
        validate: *value < 100,
        message: "not less than one hundred".to_string(),
        type_value: *value,
    }
}

fn members() -> Vec<Box<dyn Fn(&i32) -> ValidateCoreReturnType<i32>>> {
    vec![Box::new(positive), Box::new(less_than_hundred)]
}

#[test]
fn test_intersection_accepts_values_passing_all_validators() {
    let validator = umt_intersection(members());
    let result = validator(&42);
    assert!(result.validate);
    assert_eq!(result.message, "");
}

#[test]
fn test_intersection_rejects_values_failing_the_first_validator() {
    let validator = umt_intersection(members());
    let result = validator(&-1);
    assert!(!result.validate);
}

#[test]
fn test_intersection_rejects_values_failing_the_second_validator() {
    let validator = umt_intersection(members());
    let result = validator(&200);
    assert!(!result.validate);
}

#[test]
fn test_intersection_returns_the_first_failing_validators_message() {
    let validator = umt_intersection(members());
    let result = validator(&-5);
    assert!(!result.validate);
    assert_eq!(result.message, "not positive");
}

#[test]
fn test_intersection_works_with_three_or_more_validators() {
    let validator = umt_intersection(vec![
        Box::new(positive) as Box<dyn Fn(&i32) -> ValidateCoreReturnType<i32>>,
        Box::new(less_than_hundred),
        Box::new(|value: &i32| ValidateCoreReturnType {
            validate: *value % 2 == 0,
            message: "not even".to_string(),
            type_value: *value,
        }),
    ]);
    assert!(validator(&42).validate);
    assert!(!validator(&43).validate);
    assert_eq!(validator(&43).message, "not even");
    assert!(!validator(&-2).validate);
    assert_eq!(validator(&-2).message, "not positive");
}

#[test]
fn test_intersection_preserves_the_original_value_in_the_type_field() {
    let validator = umt_intersection(members());
    let ok = validator(&7);
    assert_eq!(ok.type_value, 7);
    let bad = validator(&-3);
    assert_eq!(bad.type_value, -3);
}

#[test]
fn test_intersection_works_with_string_validators() {
    let validator = umt_intersection(vec![
        Box::new(|value: &String| ValidateCoreReturnType {
            validate: !value.is_empty(),
            message: "empty".to_string(),
            type_value: value.clone(),
        }) as Box<dyn Fn(&String) -> ValidateCoreReturnType<String>>,
        Box::new(|value: &String| ValidateCoreReturnType {
            validate: value.len() <= 5,
            message: "too long".to_string(),
            type_value: value.clone(),
        }),
    ]);
    assert!(validator(&"hi".to_string()).validate);
    let empty = validator(&String::new());
    assert!(!empty.validate);
    assert_eq!(empty.message, "empty");
    let long = validator(&"hello world".to_string());
    assert!(!long.validate);
    assert_eq!(long.message, "too long");
}
