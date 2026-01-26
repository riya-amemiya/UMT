//! Tests for optional validation

use umt_rust::validate::core::ValidateCoreReturnType;
use umt_rust::validate::object::{umt_optional, umt_validate_optional};

#[test]
fn test_optional_allows_none_values() {
    let validator = umt_optional(|x: &i32| *x > 0);
    assert!(validator(&None));
}

#[test]
fn test_optional_validates_some_valid_values() {
    let validator = umt_optional(|x: &i32| *x > 0);
    assert!(validator(&Some(5)));
}

#[test]
fn test_optional_fails_for_some_invalid_values() {
    let validator = umt_optional(|x: &i32| *x > 0);
    assert!(!validator(&Some(-1)));
}

#[test]
fn test_validate_optional_none() {
    let result = umt_validate_optional(&None::<i32>, |v| ValidateCoreReturnType {
        validate: *v > 0,
        message: String::new(),
        type_value: *v,
    });
    assert!(result.validate);
    assert!(result.type_value.is_none());
}

#[test]
fn test_validate_optional_some_valid() {
    let result = umt_validate_optional(&Some(5_i32), |v| ValidateCoreReturnType {
        validate: *v > 0,
        message: String::new(),
        type_value: *v,
    });
    assert!(result.validate);
    assert_eq!(result.type_value, Some(5));
}

#[test]
fn test_validate_optional_some_invalid() {
    let result = umt_validate_optional(&Some(-1_i32), |v| ValidateCoreReturnType {
        validate: *v > 0,
        message: "Value must be positive".to_string(),
        type_value: *v,
    });
    assert!(!result.validate);
    assert_eq!(result.message, "Value must be positive");
}

#[test]
fn test_optional_with_string_validator() {
    let validator = umt_optional(|s: &String| s.len() >= 3);
    assert!(validator(&None));
    assert!(validator(&Some("hello".to_string())));
    assert!(!validator(&Some("ab".to_string())));
}

#[test]
fn test_multiple_optional_validators() {
    let positive_validator = umt_optional(|x: &i32| *x > 0);
    let even_validator = umt_optional(|x: &i32| *x % 2 == 0);

    // Both should pass for None
    assert!(positive_validator(&None));
    assert!(even_validator(&None));

    // Both should pass for Some(4)
    assert!(positive_validator(&Some(4)));
    assert!(even_validator(&Some(4)));

    // positive should pass, even should fail for Some(3)
    assert!(positive_validator(&Some(3)));
    assert!(!even_validator(&Some(3)));

    // positive should fail for Some(-2)
    assert!(!positive_validator(&Some(-2)));
}

#[test]
fn test_nested_optional_validation() {
    // Simulating nested optional objects
    let inner_validator = |opt: &Option<i32>| match opt {
        None => true,
        Some(v) => *v > 0,
    };

    let outer_validator = umt_optional(inner_validator);

    // None outer is valid
    assert!(outer_validator(&None));

    // Some(None) inner is valid
    assert!(outer_validator(&Some(None)));

    // Some(Some(5)) is valid
    assert!(outer_validator(&Some(Some(5))));

    // Some(Some(-1)) is invalid
    assert!(!outer_validator(&Some(Some(-1))));
}

// Note: The TypeScript tests include type inference tests which are handled
// at compile time in Rust through the type system. The Rust type system
// ensures correct types automatically.
