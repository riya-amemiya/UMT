//! Tests for nullable validation

use umt_rust::validate::core::ValidateCoreReturnType;
use umt_rust::validate::object::umt_nullable;

fn positive_number(value: &i32) -> ValidateCoreReturnType<&'static str> {
    ValidateCoreReturnType {
        validate: *value > 0,
        message: if *value > 0 {
            String::new()
        } else {
            "Value must be positive".to_string()
        },
        type_value: "number",
    }
}

#[test]
fn test_nullable_allows_none_values() {
    let validator = umt_nullable(positive_number);
    let result = validator(&None);
    assert!(result.validate);
    assert_eq!(result.type_value, "null");
    assert_eq!(result.message, "");
}

#[test]
fn test_nullable_validates_some_valid_values() {
    let validator = umt_nullable(positive_number);
    let result = validator(&Some(5));
    assert!(result.validate);
    assert_eq!(result.type_value, "number");
}

#[test]
fn test_nullable_fails_for_some_invalid_values() {
    let validator = umt_nullable(positive_number);
    let result = validator(&Some(-1));
    assert!(!result.validate);
    assert_eq!(result.type_value, "number");
    assert_eq!(result.message, "Value must be positive");
}

#[test]
fn test_nullable_with_string_validator() {
    let validator = umt_nullable(|value: &String| ValidateCoreReturnType {
        validate: value.len() >= 3,
        message: String::new(),
        type_value: "string",
    });
    assert!(validator(&None).validate);
    assert!(validator(&Some("hello".to_string())).validate);
    assert!(!validator(&Some("ab".to_string())).validate);
}

#[test]
fn test_nullable_delegates_message_for_some() {
    let validator = umt_nullable(|value: &i32| ValidateCoreReturnType {
        validate: false,
        message: format!("rejected {}", value),
        type_value: "number",
    });
    let result = validator(&Some(7));
    assert!(!result.validate);
    assert_eq!(result.message, "rejected 7");
}

#[test]
fn test_nullable_is_reusable_across_calls() {
    let validator = umt_nullable(positive_number);
    let first = validator(&Some(3));
    let second = validator(&None);
    assert!(first.validate);
    assert_eq!(first.type_value, "number");
    assert!(second.validate);
    assert_eq!(second.type_value, "null");
}
